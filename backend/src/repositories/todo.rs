use super::label::Label;
use super::RepositoryError;
use anyhow::Ok;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use validator::Validate;

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity>;
    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity>;
    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TodoFromRow {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TodoWithLabelFromRow {
    id: i32,
    text: String,
    completed: bool,
    label_id: Option<i32>,
    label_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

fn fold_entities(rows: Vec<TodoWithLabelFromRow>) -> Vec<TodoEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<TodoEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let mut todos = accum.iter_mut();
        while let Some(todo) = todos.next() {
            if todo.id == row.id {
                todo.labels.push(Label {
                    id: row.label_id.unwrap(),
                    name: row.label_name.clone().unwrap(),
                });
                continue 'outer;
            }
        }

        let labels = if row.label_id.is_some() {
            vec![Label {
                id: row.label_id.unwrap(),
                name: row.label_name.clone().unwrap(),
            }]
        } else {
            vec![]
        };

        accum.push(TodoEntity {
            id: row.id,
            text: row.text.clone(),
            completed: row.completed,
            labels,
        });
    }
    accum
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: String,
    labels: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: Option<String>,
    completed: Option<bool>,
    labels: Option<Vec<i32>>,
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, TodoFromRow>(
            r#"insert into todos (text, completed) values ($1, false) returning *;"#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await
        .expect("cannot insert into todos table");

        sqlx::query(
            r#"insert into todo_labels (todo_id, label_id) select $1, id from unnest($2) as t(id);"#,
        )
        .bind(row.id)
        .bind(payload.labels)
        .execute(&self.pool)
        .await
        .expect("cannot insert into todo_labels table");

        tx.commit().await.expect("cannot commit transaction");

        let todo = self.find(row.id).await.expect("cannot find committed todo");
        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"select todos.*, labels.id as label_id, labels.name as label_name from todos left outer join todo_labels tl on todos.id = tl.todo_id left outer join labels on labels.id = tl.label_id where todos.id=$1;"#
        ).bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        }).expect("cannot find items");

        let todos = fold_entities(items);
        let todo = todos.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(todo.clone())
    }

    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"select todos.*, labels.id as label_id, labels.name as label_name from todos left outer join todo_labels tl on todos.id = tl.todo_id left outer join labels on labels.id = tl.label_id order by todos.id desc;"#,
        ).fetch_all(&self.pool).await?;
        Ok(fold_entities(items))
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;
        let old_todo = self.find(id).await?;
        sqlx::query(r#"update todos set text=$1, completed=$2 where id = $3 returning *;"#)
            .bind(payload.text.unwrap_or(old_todo.text))
            .bind(payload.completed.unwrap_or(old_todo.completed))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        if let Some(labels) = payload.labels {
            sqlx::query(r#"delete from todo_labels where todo_id=$1;"#)
                .bind(id)
                .execute(&self.pool)
                .await?;
            sqlx::query(
                r#"insert into todo_labels (todo_id, label_id) select $1, id from unnest ($2) as t(id);"#,
            ).bind(id).bind(labels).execute(&self.pool).await?;
        };

        tx.commit().await?;
        let todo = self.find(id).await?;

        Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let tx = self.pool.begin().await?;
        // delete todo_label
        sqlx::query(r#"delete from todo_labels where todo_id=$1;"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;
        // delete todo
        sqlx::query(r#"delete from todos where id=$1;"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::{env, vec};

    #[test]
    fn fold_entities_test() {
        let label_1 = Label {
            id: 1,
            name: String::from("label 1"),
        };
        let label_2 = Label {
            id: 2,
            name: String::from("label 2"),
        };
        let rows = vec![
            TodoWithLabelFromRow {
                id: 1,
                text: String::from("todo 1"),
                completed: false,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
            },
            TodoWithLabelFromRow {
                id: 1,
                text: String::from("todo 1"),
                completed: false,
                label_id: Some(label_2.id),
                label_name: Some(label_2.name.clone()),
            },
            TodoWithLabelFromRow {
                id: 2,
                text: String::from("todo 2"),
                completed: false,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
            },
        ];
        let res = fold_entities(rows);
        assert_eq!(
            res,
            vec![
                TodoEntity {
                    id: 1,
                    text: String::from("todo 1"),
                    completed: false,
                    labels: vec![label_1.clone(), label_2],
                },
                TodoEntity {
                    id: 2,
                    text: String::from("todo 2"),
                    completed: false,
                    labels: vec![label_1],
                }
            ]
        );
    }

    #[cfg(feature = "database-test")]
    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
        let pool = PgPool::connect(database_url)
            .await
            .unwrap_or_else(|_| panic!("fail connect database, url is [{}]", database_url));

        // prepare label data
        let label_name = String::from("[crud_scenario] test label");
        let optional_label = sqlx::query_as::<_, Label>(r#"select * from labels where name = $1;"#)
            .bind(label_name.clone())
            .fetch_optional(&pool)
            .await
            .expect("Failed to prepare label data.");
        let label_1 = if let Some(label) = optional_label {
            label
        } else {
            sqlx::query_as::<_, Label>(r#"insert into labels ( name ) values ( $1 ) returning *;"#)
                .bind(label_name)
                .fetch_one(&pool)
                .await
                .expect("Failed to insert label data.")
        };

        // prepare todo data
        let repository = TodoRepositoryForDb::new(pool.clone());
        let todo_text = "[crud_scenario] text";

        // create
        let created = repository
            .create(CreateTodo::new(todo_text.to_string(), vec![label_1.id]))
            .await
            .expect("[create] returned Err");
        assert_eq!(created.text, todo_text);
        assert!(!created.completed);
        assert_eq!(*created.labels.first().unwrap(), label_1);

        // find
        let todo = repository
            .find(created.id)
            .await
            .expect("[find] returned Err");
        assert_eq!(created, todo);

        // all
        let todos = repository.all().await.expect("[all] returned Err");
        let todo = todos.first().unwrap();
        assert_eq!(created, *todo);

        // update
        let updated_text = "[crud_scenario] updated text";
        let todo = repository
            .update(
                todo.id,
                UpdateTodo {
                    text: Some(updated_text.to_string()),
                    completed: Some(true),
                    labels: Some(vec![]),
                },
            )
            .await
            .expect("[update] returned Err");
        assert_eq!(created.id, todo.id);
        assert_eq!(todo.text, updated_text);
        assert!(todo.labels.is_empty());

        // delete
        repository
            .delete(todo.id)
            .await
            .expect("[delete] returned Err");
        let res = repository.find(created.id).await;

        assert!(res.is_err());

        // todo
        let todo_rows = sqlx::query(r#"select * from todos where id=$1"#)
            .bind(todo.id)
            .fetch_all(&pool)
            .await
            .expect("[delete] todo_labels fetch error");
        assert!(todo_rows.is_empty());

        // todo_label
        let rows = sqlx::query(r#"select * from todo_labels where todo_id=$1 "#)
            .bind(todo.id)
            .fetch_all(&pool)
            .await
            .expect("[delete] todo_labels fetch error");
        assert!(rows.is_empty());

        // clean test data from db
        sqlx::query(r#"delete from labels where id=$1"#)
            .bind(label_1.id)
            .execute(&pool)
            .await
            .expect("[delete] Failed to delete prepared label data.");
        let res = sqlx::query(r#"select * from labels where id=$1"#)
            .bind(label_1.id)
            .fetch_all(&pool)
            .await
            .expect("[delete] labels fetch error");
        assert!(res.is_empty());
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use anyhow::Context;
    use axum::async_trait;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    impl TodoEntity {
        pub fn new(id: i32, text: String) -> Self {
            Self {
                id,
                text,
                completed: false,
                labels: vec![],
            }
        }
    }

    impl CreateTodo {
        pub fn new(text: String, labels: Vec<i32>) -> Self {
            Self { text, labels }
        }
    }

    type TodoData = HashMap<i32, TodoEntity>;

    #[derive(Debug, Clone)]
    pub struct TodoRepositoryForMemory {
        store: Arc<RwLock<TodoData>>,
    }

    impl TodoRepositoryForMemory {
        pub fn new() -> Self {
            TodoRepositoryForMemory {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryForMemory {
        async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let todo = TodoEntity::new(id, payload.text);
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
            let store = self.read_store_ref();
            let todo = store
                .get(&id)
                .cloned()
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self) -> anyhow::Result<Vec<TodoEntity>> {
            let store = self.read_store_ref();
            let todos = Vec::from_iter(store.values().cloned());
            Ok(todos)
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
            let text = payload.text.unwrap_or(todo.text.clone());
            let completed = payload.completed.unwrap_or(todo.completed);
            let todo = TodoEntity {
                id,
                text,
                completed,
                labels: vec![],
            };
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> anyhow::Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn todo_crud_scenario() {
            let text = "todo text".to_string();
            let id = 1;
            let labels = vec![];
            let expected = TodoEntity::new(id, text.clone());

            // create
            let repository = TodoRepositoryForMemory::new();
            let todo = repository
                .create(CreateTodo { text, labels })
                .await
                .expect("failed create todo");
            assert_eq!(expected, todo);

            // find
            let todo = repository.find(todo.id).await.unwrap();
            assert_eq!(expected, todo);

            // all
            let todos = repository.all().await.expect("failed get all todo");
            assert_eq!(vec![expected], todos);

            // update
            let text = "update todo text".to_string();
            let todo = repository
                .update(
                    1,
                    UpdateTodo {
                        text: Some(text.clone()),
                        completed: Some(true),
                        labels: Some(vec![]),
                    },
                )
                .await
                .expect("failed update todo.");
            assert_eq!(
                TodoEntity {
                    id,
                    text,
                    completed: true,
                    labels: vec![],
                },
                todo
            );

            // delete
            let res = repository.delete(id).await;
            assert!(res.is_ok());
        }
    }
}

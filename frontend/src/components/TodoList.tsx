import { Card, Checkbox, Stack, Typography } from "@mui/material";
import React, { FC } from "react";
import { Todo } from "../types/todo";
import TodoItem from "./TodoItem";

type Props = {
    todos: Todo[];
    onUpdate: (todo: Todo) => void;
    onDelete: (id: number) => void;
};

const TodoList: FC<Props> = (props: Props) => {
    return (
        <Stack spacing={2}>
            <Typography variant="h2">Todo List</Typography>
            <Stack spacing={2}>
                {props.todos.map((todo) => (
                    <TodoItem
                        key={todo.id}
                        todo={todo}
                        onUpdate={props.onUpdate}
                        onDelete={props.onDelete}
                    />
                ))}
            </Stack>
        </Stack>
    );
};

export default TodoList;

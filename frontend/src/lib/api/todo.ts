import { NewTodoPayload, Todo } from "../../types/todo";

const URL = "http://localhost:3000";
const HEADER = { "Content-Type": "application/json" };

export const addTodoItem = async (payload: NewTodoPayload) => {
    const res = await fetch(`${URL}/todos`, {
        method: "POST",
        headers: HEADER,
        body: JSON.stringify(payload),
    });
    if (!res.ok) {
        throw new Error("add todo request failed");
    }
    const json: Todo = await res.json();
    return json;
};

export const getTodoItems = async () => {
    const res = await fetch(`${URL}/todos`);
    if (!res.ok) {
        throw new Error("get todo request failed");
    }
    const json: Todo[] = await res.json();
    return json;
};

export const updateTodoItem = async (todo: Todo) => {
    const { id, ...updateTodo } = todo;
    const res = await fetch(`${URL}/todos/${id}`, {
        method: "PATCH",
        headers: HEADER,
        body: JSON.stringify(updateTodo),
    });
    if (!res.ok) {
        throw new Error("update todo request failed");
    }
    const json: Todo = await res.json();
    return json;
};

export const deleteTodoItem = async (id: number) => {
    const res = await fetch(`${URL}/todos/${id}`, { method: "DELETE" });
    if (!res.ok) {
        throw new Error("delete todo request failed");
    }
};

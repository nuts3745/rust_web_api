import { Card, Checkbox, Stack, Typography } from "@mui/material";
import React, { FC } from "react";
import { Todo } from "../types/todo";

type Props = {
    todos: Todo[];
    onUpdate: (todo: Todo) => void;
};

const TodoList: FC<Props> = (props: Props) => {
    const handleCompletedCheckbox = (todo: Todo) => {
        props.onUpdate({
            ...todo,
            completed: !todo.completed,
        });
    };

    return (
        <Stack spacing={2}>
            <Typography variant="h2">Todo List</Typography>
            <Stack spacing={2}>
                {props.todos.map((todo) => (
                    <Card key={todo.id} sx={{ p: 2 }}>
                        <Stack direction="row" alignItems="center">
                            <Checkbox
                                checked={todo.completed}
                                onChange={() => handleCompletedCheckbox(todo)}
                            />
                            <Typography>{todo.text}</Typography>
                        </Stack>
                    </Card>
                ))}
            </Stack>
        </Stack>
    );
};

export default TodoList;

import {
    Card,
    Stack,
    Checkbox,
    Typography,
    Grid,
    Button,
    Modal,
    TextField,
    Box,
} from "@mui/material";
import React, { ChangeEventHandler, FC, useEffect, useState } from "react";
import { modalInnerStyle } from "../styles/modal";
import { Todo } from "../types/todo";

type Props = {
    todo: Todo;
    onUpdate: (todo: Todo) => void;
    onDelete: (id: number) => void;
};

const TodoItem: FC<Props> = (props: Props) => {
    const [editing, setEditing] = useState(false);
    const [editText, setEditText] = useState("");

    useEffect(() => {
        setEditText(props.todo.text);
    }, [props.todo]);

    const handleCompletedCheckbox: ChangeEventHandler = (e) => {
        props.onUpdate({
            ...props.todo,
            completed: !props.todo.completed,
        });
    };

    const handleDelete = () => {
        props.onDelete(props.todo.id);
    };

    const onCloseEditModal = () => {
        props.onUpdate({ ...props.todo, text: editText });
        setEditing(false);
    };

    return (
        <>
            <Card sx={{ p: 1 }}>
                <Grid container spacing={2} alignItems="center">
                    <Grid item xs={1}>
                        <Checkbox
                            checked={props.todo.completed}
                            onChange={handleCompletedCheckbox}
                        />
                    </Grid>
                    <Grid item xs={8}>
                        <Stack spacing={1}>
                            <Typography variant="caption" fontSize={16}>
                                {props.todo.text}
                            </Typography>
                        </Stack>
                    </Grid>
                    <Grid item xs={2}>
                        <Stack direction="row" spacing={1}>
                            <Button
                                onClick={() => setEditing(true)}
                                color="info">
                                Edit
                            </Button>
                            <Button onClick={handleDelete} color="error">
                                delete
                            </Button>
                        </Stack>
                    </Grid>
                </Grid>
                <Modal open={editing} onClose={onCloseEditModal}>
                    <Box sx={modalInnerStyle}>
                        <Stack spacing={2}>
                            <TextField
                                size="small"
                                label="todo text"
                                defaultValue={props.todo.text}
                                onChange={(e) => setEditText(e.target.value)}
                            />
                        </Stack>
                    </Box>
                </Modal>
            </Card>
        </>
    );
};

export default TodoItem;

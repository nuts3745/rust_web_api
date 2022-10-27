import { Box, Button, Grid, Paper, TextField } from "@mui/material";
import { FC, useState } from "react";
import { NewTodoPayload } from "../types/todo";

type Props = {
    onSubmit: (newTodo: NewTodoPayload) => void;
};

const TodoForm: FC<Props> = (props: Props) => {
    const [editText, setEditText] = useState("");
    const addTodoHandler = async () => {
        if (!editText) return;

        props.onSubmit({ text: editText });
        setEditText("");
    };

    return (
        <Paper elevation={2}>
            <Box sx={{ p: 2 }}>
                <Grid container rowSpacing={2} columnSpacing={5}>
                    <Grid item xs={12}>
                        <TextField
                            label="new todo text"
                            variant="filled"
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            fullWidth
                        />
                    </Grid>
                    <Grid item xs={9} />
                    <Grid item xs={3}>
                        <Button onClick={addTodoHandler} fullWidth>
                            Add Todo
                        </Button>
                    </Grid>
                </Grid>
            </Box>
        </Paper>
    );
};

export default TodoForm;

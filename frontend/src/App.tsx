import { FC, useState } from 'react'
import 'modern-css-reset'
import { createTheme, ThemeProvider } from '@mui/material'
import { NewTodoPayload, Todo } from './types/todo'

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([])
  const createId = () => todos.length + 1

  const onSubmit = async (payload: NewTodoPayload) => {

  }

  return <></>
}

const theme = createTheme({
  typography: {
    h1: {
      fontSize: 30,
    },
    h2: {
      fontSize: 20,
    },
  },
})

const App: FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <TodoApp />
    </ThemeProvider>
  )
}

export default App

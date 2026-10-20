# TODO app 
- This will be the todo apps with project based 

## Flow
- You create an new project, each project will have the

```ts
class Project {
  deadline: DateTime
  tasks: Task[]
}

enum Status {
  ToDo,
  InProgress,
  Done
}

class Task {
  title: string
  status: Status
  deadline: DateTime
  subTasks: Task[]
}
```
- Then settings it with each task
- The calendar will notify me when any thing got deadline

## App technology
- Backend: Axum
- DB: SQLite
- SQLx
- Frontend: JS

## what next ?
- [ ] Done the first backend works, core logic
- [ ] Testing the backend with db interactive
- [ ] Guide the frontend 
- [ ] Connect frontend with backend route
- [ ] Deployment

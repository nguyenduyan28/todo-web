pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use chrono::{DateTime, Utc};

pub type DateTimeUtc = DateTime<Utc>;

#[derive(Debug)]
pub struct Project {
    dead_line: Option<DateTimeUtc>,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    title: String,
    status: Status,
    dead_line: Option<DateTimeUtc>,
    sub_task: Vec<Task>,
}

impl Project {
    pub fn new (dead_line: Option<DateTimeUtc>, tasks: Vec<Task>) -> Self {
        Self {
            dead_line: dead_line,
            tasks: tasks,
        }
    }
    pub fn set_dead_line (&mut self, dead_line: Option<DateTimeUtc>) {
        self.dead_line = dead_line;
    }

    pub fn set_tasks (&mut self, tasks: &Vec<Task>) {
        self.tasks = tasks.to_vec();
    }

    pub fn get_dead_line (&self) -> Option<DateTimeUtc> {
        self.dead_line
    }

    pub fn get_task (&self) -> &Vec<Task> {
        &self.tasks
    }
}

impl Task {
    pub fn new (title: String, status: Status, dead_line: Option<DateTimeUtc>, sub_task: Vec<Task>) -> Task {
        Self {
            title,
            status,
            dead_line,
            sub_task
        }
    }
    /// setter
    pub fn set_title (&mut self, title: String) {
        self.title = title;
    }

    pub fn set_status (&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_dead_line (&mut self, dead_line: Option<DateTimeUtc>) {
        self.dead_line = dead_line;
    }

    pub fn set_sub_task (&mut self, sub_task: Vec<Task>) {
        self.sub_task = sub_task;
    }

    /// getter
    pub fn get_title (&self) -> &str {
        &self.title
    }

    pub fn get_status (&self) -> &Status {
        &self.status
    }

    pub fn get_dead_line (&self) -> &Option<DateTimeUtc> {
        &self.dead_line
    }

    pub fn get_sub_task (&self) -> &Vec<Task> {
        &self.sub_task
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn new_project() {
        let time: Option<DateTimeUtc> = Some(Utc::now());
        let tasks: Vec<Task> = Vec::new();
        let project: Project = Project::new(time, tasks);
        assert_eq!(project.dead_line, time);
        assert_eq!(project.tasks.len(), 0);
    }

    #[test]
    fn set_tasks() {
        let time: Option<DateTimeUtc> = Some(Utc::now());
        let task1: Task = Task {
            title: "learn rust".to_string(),
            status: Status::ToDo,
            dead_line: Some(Utc::now()),
            sub_task: Vec::new(),
        };
        let mut project: Project = Project::new(time, Vec::new());
        let tasks_copy = task1.clone();
        let tasks: Vec<Task> = vec![tasks_copy];
        project.set_tasks(&tasks);
        assert_eq!(project.tasks[0], task1);
        let task_result = project.get_task();
        assert_eq!(task_result.to_vec(), tasks);
    }
}

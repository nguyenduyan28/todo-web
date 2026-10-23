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

#[derive(Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
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
        println!("project {:#?}", project);
        assert_eq!(project.dead_line, time);
        assert_eq!(project.tasks.len(), 0);
    }
}

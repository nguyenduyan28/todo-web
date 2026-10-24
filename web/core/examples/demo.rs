use core::*;
use chrono::{Utc};

fn main() {
    let time: Option<DateTimeUtc> = Some(Utc::now());
    let task1: Task = Task {
        title: "learn rust".to_string(),
        status: Status::ToDo,
        dead_line: Some(Utc::now()),
        sub_task: Vec::new(),
    };
    let mut project: Project = Project::new(time, Vec::new());
    let tasks: Vec<Task> = vec![task1];
    project.set_tasks(tasks);
    println!("project {:?}", project);
}

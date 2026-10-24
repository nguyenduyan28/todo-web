use core::*;
use chrono::{Utc};

fn main() {
    let time: Option<DateTimeUtc> = Some(Utc::now());
    let task1: Task = Task::new (
        "learn_rust".to_string(),
        Status::ToDo,
        Some(Utc::now()),
        Vec::new()
    );
    let mut project: Project = Project::new(time, Vec::new());
    let tasks: Vec<Task> = vec![task1.clone()];
    project.set_tasks(&tasks);
    println!("project {:?}", project);
}

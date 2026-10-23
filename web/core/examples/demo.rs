use core::*;
use chrono::{Utc};

fn main() {
    let time: Option<DateTimeUtc> = Some(Utc::now());
    let tasks: Vec<Task> = Vec::new();
    let project: Project = Project::new(time, tasks);
    println!("project {:#?}", project);
}

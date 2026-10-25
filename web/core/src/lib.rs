use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime};

pub type DateTimeUtc = NaiveDateTime;

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
    fn new_project() {
        let time: DateTime<Utc> = Utc::now();
        let naive_datetime = time.naive_utc();
        let tasks: Vec<Task> = Vec::new();
        let project: Project = Project::new(Some(naive_datetime), tasks);
        assert_eq!(project.dead_line, Some(naive_datetime));
        assert_eq!(project.tasks.len(), 0);
    }

    #[test]
    fn set_tasks() {
        let time: DateTime<Utc> = Utc::now();
        let naive_datetime = time.naive_utc();
        let task1: Task = Task {
            title: "learn rust".to_string(),
            status: Status::ToDo,
            dead_line: Some(naive_datetime),
            sub_task: Vec::new(),
        };
        let mut project: Project = Project::new(Some(naive_datetime), Vec::new());
        let tasks_copy = task1.clone();
        let tasks: Vec<Task> = vec![tasks_copy];
        project.set_tasks(&tasks);
        assert_eq!(project.tasks[0], task1);
        let task_result = project.get_task();
        assert_eq!(task_result.to_vec(), tasks);
    }

    #[test]
    fn get_dead_lines() {
        let time_str = "2026-10-25";
        let parsed_date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").expect("Failed to parse");

        let dt = parsed_date.and_hms_opt(14, 30, 0).expect("Failed to parse time");
        let time: DateTime<Utc> = Utc::now();

        let naive_datetime = time.naive_utc();
        let mut project = Project::new(Some(naive_datetime), Vec::new());

        project.set_dead_line(Some(dt));
        let dl = project.get_dead_line();
        assert_eq!(dl, Some(dt));
    }
}

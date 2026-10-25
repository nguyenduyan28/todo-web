use core::*;
use chrono::{Utc, DateTime, NaiveDate};

fn main() {
    let time_str = "2026-10-30";

    let parsed_date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").expect("Failed to parse");
    let dt = parsed_date.and_hms_opt(14, 30, 0).expect("Failed to parse time");

    let time: DateTime<Utc> = Utc::now();
    let naive_datetime = time.naive_utc();

    let mut project = Project::new(Some(naive_datetime), Vec::new());
    println!("Current deadline of the project {:#?}", project.get_dead_line().unwrap());
    project.set_dead_line(Some(dt));
    println!("Current deadline of the project {:#?}", project.get_dead_line().unwrap());
}

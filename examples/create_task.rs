extern crate task_hookrs;
extern crate chrono;
extern crate serde_json;
extern crate uuid;

use task_hookrs::task::Task;
use task_hookrs::status::TaskStatus;
use task_hookrs::uda::UDA;

use chrono::NaiveDateTime;
use serde_json::to_string;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::nil();
    let date = NaiveDateTime::parse_from_str("2016-12-31 12:13:14", "%Y-%m-%d %H:%M:%S").unwrap();
    let t = Task::new(
        Some(12),
        TaskStatus::Pending,
        uuid,
        date.into(),
        "Test task".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        UDA::default(),
    );
    println!("[{}]", to_string(&t).unwrap());
}

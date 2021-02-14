extern crate task_hookrs;
extern crate chrono;
extern crate serde_json;
extern crate uuid;

use std::io::stdin;

use task_hookrs::status::TaskStatus;
use task_hookrs::import::import;

fn main() {
    let mut tasks = import(stdin()).unwrap();
    assert_eq!(tasks.len(), 1);
    let t = tasks.pop().unwrap();
    assert_eq!(*t.status(), TaskStatus::Pending);
    assert_eq!(*t.description(), "Test task".to_owned());
    assert_eq!(t.priority(), None);
    assert_eq!(t.uda().get("priority"), None);

    println!("Successfully imported:\n{:?}", t);
}

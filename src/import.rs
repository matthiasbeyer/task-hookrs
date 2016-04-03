use std::io::Read;

use serde_json;

use result::Result;
use task::Task;
use error::{TaskError, TaskErrorKind};

pub fn import<R: Read>(r: R) -> Result<Vec<Task>> {
    serde_json::from_reader(r)
        .map_err(|e| {
            TaskError::new(TaskErrorKind::ParserError, Some(Box::new(e)))
        })
}

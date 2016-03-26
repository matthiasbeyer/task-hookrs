use std::result::Result as RResult;

use error::TaskError;

pub type Result<T> = RResult<T, TaskError>;

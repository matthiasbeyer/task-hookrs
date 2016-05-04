//! Module containing `Result` type

use std::result::Result as RResult;

use error::TaskError;

/// Result type wrapping the standard std::result::Result type
pub type Result<T> = RResult<T, TaskError>;

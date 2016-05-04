//! Module containing `TaskStatus` type and trait impls

use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;
use serde::de::Visitor;

/// Enum for status taskwarrior supports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    /// Pening status type
    Pending,

    /// Deleted status type
    Deleted,

    /// Completed status type
    Completed,

    /// Waiting status type
    Waiting,

    /// Recurring status type
    Recurring
}

impl Display for TaskStatus {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &TaskStatus::Pending   => write!(fmt, "Pending"),
            &TaskStatus::Deleted   => write!(fmt, "Deleted"),
            &TaskStatus::Completed => write!(fmt, "Completed"),
            &TaskStatus::Waiting   => write!(fmt, "Waiting"),
            &TaskStatus::Recurring => write!(fmt, "Recurring"),
        }
    }
}

impl Serialize for TaskStatus {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_str(
            match self {
                &TaskStatus::Pending   => "pending",
                &TaskStatus::Deleted   => "deleted",
                &TaskStatus::Completed => "completed",
                &TaskStatus::Waiting   => "waiting",
                &TaskStatus::Recurring => "recurring",
            }
        )
    }

}

impl Deserialize for TaskStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<TaskStatus, D::Error>
        where D: Deserializer
    {
        struct TaskStatusVisitor;

        impl Visitor for TaskStatusVisitor {
            type Value = TaskStatus;

            fn visit_str<E>(&mut self, value: &str) -> Result<TaskStatus, E>
                where E: Error
            {
                match value {
                    "pending"   => Ok(TaskStatus::Pending),
                    "deleted"   => Ok(TaskStatus::Deleted),
                    "completed" => Ok(TaskStatus::Completed),
                    "waiting"   => Ok(TaskStatus::Waiting),
                    "recurring" => Ok(TaskStatus::Recurring),
                    _           => Err(Error::custom("expected one of pending, deleted, completed, waiting, recurring")),
                }
            }
        }

        deserializer.deserialize(TaskStatusVisitor)
    }
}


use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;
use serde::de::Visitor;

use std::convert::Into;
use std::convert::From;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Deleted,
    Completed,
    Waiting,
    Recurring
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


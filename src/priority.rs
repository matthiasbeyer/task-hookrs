//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing TaskPriority types and trait impls

use serde::Serialize;
use serde::ser::Serializer;
use serde::de::Deserialize;
use serde::de::Deserializer;
use serde::Error;
use serde::de::Visitor;

/// Enum for the priorities taskwarrior supports.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    /// Low prio for a Task
    Low,

    /// Medium prio for a Task
    Medium,

    /// High prio for a Task
    High,
}


impl Serialize for TaskPriority {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_str(
            match self {
                &TaskPriority::Low    => "L",
                &TaskPriority::Medium => "M",
                &TaskPriority::High   => "H",
            }
        )
    }

}

impl Deserialize for TaskPriority {

    fn deserialize<D>(deserializer: &mut D) -> Result<TaskPriority, D::Error>
        where D: Deserializer
    {
        struct TaskPriorityVisitor;

        impl Visitor for TaskPriorityVisitor {
            type Value = TaskPriority;

            fn visit_str<E>(&mut self, value: &str) -> Result<TaskPriority, E>
                where E: Error
            {
                match value {
                    "L" => Ok(TaskPriority::Low),
                    "M" => Ok(TaskPriority::Medium),
                    "H" => Ok(TaskPriority::High),
                    _ => Err(Error::custom("expected one of 'L', 'M', 'H'")),
                }
            }
        }

        deserializer.deserialize(TaskPriorityVisitor)
    }
}


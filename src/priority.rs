use serde::Serialize;
use serde::ser::Serializer;
use serde::de::Deserialize;
use serde::de::Deserializer;
use serde::Error;
use serde::de::Visitor;

/// Enum for the priorities taskwarrior supports.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
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


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

impl TaskStatus {

    pub fn from_str(s: &str) -> Option<TaskStatus> {
        match s {
            "pending"   => Some(TaskStatus::Pending),
            "deleted"   => Some(TaskStatus::Deleted),
            "completed" => Some(TaskStatus::Completed),
            "waiting"   => Some(TaskStatus::Waiting),
            "recurring" => Some(TaskStatus::Recurring),
            _           => None,
        }
    }

}

impl Into<String> for TaskStatus {

    fn into(self) -> String {
        String::from(match self {
            TaskStatus::Pending   => "pending",
            TaskStatus::Deleted   => "deleted",
            TaskStatus::Completed => "completed",
            TaskStatus::Waiting   => "waiting",
            TaskStatus::Recurring => "recurring",
        })
    }

}


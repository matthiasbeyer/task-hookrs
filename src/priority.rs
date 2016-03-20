#[derive(Debug, Clone)]
pub enum TaskPriority {
    NoPrio,
    Low,
    Medium,
    High,
}

impl<'a> From<&'a str> for TaskPriority {

    fn from(s: &str) -> TaskPriority {
        match s {
            "L" => TaskPriority::Low,
            "M" => TaskPriority::Medium,
            "H" => TaskPriority::High,
            _ => TaskPriority::NoPrio,
        }
    }
}

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

impl Into<String> for TaskPriority {

    fn into(self) -> String {
        String::from(match self {
            TaskPriority::Low    => "L",
            TaskPriority::Medium => "M",
            TaskPriority::High   => "H",
            TaskPriority::NoPrio => "",
        })
    }

}

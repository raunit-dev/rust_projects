#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub deadline: Option<String>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deadline = self.deadline.as_deref().unwrap_or("No Deadline");
        let status = match self.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Done",
        };
        let priority = match self.priority {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        write!(
            f,
            "[#{}] {} [{}] - {} (Due: {})",
            self.id, self.title, status, priority, deadline
        )
    }
}

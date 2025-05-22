use crate::manager::TaskManager;
use crate::task::Priority;

pub enum Command {
    Add { title: String, priority: Priority, deadline: Option<String> },
    List,
    Complete(usize),
    Delete(usize),
    Help,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.get(0).map(|s| s.to_lowercase()) {
            Some(cmd) if cmd == "add" => {
                if parts.len() < 3 {
                    return Err("Usage: add <title> <priority> [deadline]".into());
                }
                let title = parts[1].to_string();
                let priority = match parts[2].to_lowercase().as_str() {
                    "low" => Priority::Low,
                    "medium" => Priority::Medium,
                    "high" => Priority::High,
                    _ => return Err("Invalid priority. Use low, medium, or high.".into()),
                };
                let deadline = if parts.len() > 3 {
                    Some(parts[3..].join(" "))
                } else {
                    None
                };
                Ok(Command::Add { title, priority, deadline })
            }
            Some(cmd) if cmd == "list" => Ok(Command::List),
            Some(cmd) if cmd == "complete" => {
                if parts.len() != 2 {
                    return Err("Usage: complete <id>".into());
                }
                parts[1].parse().map(Command::Complete).map_err(|_| "Invalid ID".into())
            }
            Some(cmd) if cmd == "delete" => {
                if parts.len() != 2 {
                    return Err("Usage: delete <id>".into());
                }
                parts[1].parse().map(Command::Delete).map_err(|_| "Invalid ID".into())
            }
            Some(cmd) if cmd == "help" => Ok(Command::Help),
            _ => Err("Unknown command".into()),
        }
    }

    pub fn execute(&self, manager: &mut TaskManager) -> Result<(), String> {
        match self {
            Command::Add { title, priority, deadline } => {
                let task = manager.add_task(title.clone(), priority.clone(), deadline.clone());
                println!("Added task: {}", task);
                Ok(())
            }
            Command::List => {
                manager.list_tasks();
                Ok(())
            }
            Command::Complete(id) => manager.mark_complete(*id),
            Command::Delete(id) => manager.delete_task(*id),
            Command::Help => {
                println!("Available commands:");
                println!("  add <title> <priority: low|medium|high> [deadline]");
                println!("  list");
                println!("  complete <id>");
                println!("  delete <id>");
                println!("  help");
                println!("  exit");
                Ok(())
            }
        }
    }
}

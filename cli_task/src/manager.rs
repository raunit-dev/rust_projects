use crate::task::{Task, TaskStatus, Priority};

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
//static function with self cannot access the instance fields
//if i dont use self the methode cannot access or change the structs data 
    pub fn add_task(
        &mut self,
        title: String,
        priority: Priority,
        deadline: Option<String>,
    ) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            priority,
            status: TaskStatus::Pending,
            deadline,
        };
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                println!("{}", task);
            }
        }
    }

    pub fn mark_complete(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.status = TaskStatus::Completed;
                Ok(())
            }
            None => Err("Task not found".to_string()),
        }
    }

    pub fn delete_task(&mut self, id: usize) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id); // retain wala hata dega TASK vector se 
        if self.tasks.len() < initial_len {
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }
}

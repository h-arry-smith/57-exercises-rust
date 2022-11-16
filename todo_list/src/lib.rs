use std::io::{BufRead, Write};
use std::{fs::File, io::BufReader};

pub struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    pub fn load(path: &str) -> TodoList {
        let f = File::open(path).expect("Could not open file");
        let file = BufReader::new(f);

        let mut highest_id = 0;
        let mut tasks = Vec::new();

        for line in file.lines() {
            if let Ok(line) = line {
                let task = Task::parse(line.to_string());

                if task.id > highest_id {
                    highest_id = task.id
                }

                tasks.push(task);
            }
        }

        TodoList {
            tasks,
            next_id: highest_id + 1,
        }
    }

    pub fn save(&self, path: &str) {
        let mut f = File::create(path).expect("Could not open file");

        for task in self.tasks.iter() {
            f.write_all(task.to_string().as_bytes()).unwrap();
            f.write_all("\n".as_bytes()).unwrap();
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(Task {
            id: self.next_id,
            completed: false,
            task: task.to_string(),
        });
        self.next_id += 1;
    }

    pub fn show_tasks(&self) {
        for task in self.tasks.iter() {
            println!("{}", task.to_string());
        }
    }

    pub fn show_done(&self) {
        for task in self.tasks.iter().filter(|t| t.completed) {
            println!("{}", task.to_string());
        }
    }

    pub fn show_todo(&self) {
        for task in self.tasks.iter().filter(|t| !t.completed) {
            println!("{}", task.to_string());
        }
    }

    pub fn dotask(&mut self, id: &usize) {
        let task = self.tasks.iter_mut().find(|t| t.id == *id);
        match task {
            Some(task) => {
                task.complete();
            }
            None => {
                println!("Could not find task with id: {id}");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Task {
    id: usize,
    completed: bool,
    task: String,
}

impl Task {
    fn parse(line: String) -> Task {
        let (id, rest) = line.split_once(" ").unwrap();
        let mut completed = rest.trim().to_string();
        let rest = completed.split_off(3);

        let id = id.parse().expect("Invalid ID");
        let completed = match completed.as_str() {
            "[ ]" => false,
            "[X]" => true,
            _ => false,
        };

        Task {
            id,
            completed,
            task: rest.trim().to_string(),
        }
    }

    fn to_string(&self) -> String {
        let completion_marker = match self.completed {
            true => "[X]",
            false => "[ ]",
        };

        format!("{} {} {}", self.id, completion_marker, self.task)
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

#[cfg(test)]
mod tests {
    mod task {
        use crate::Task;

        #[test]
        fn not_complete() {
            assert_eq!(
                Task::parse("1 [ ] Make toast".to_string()),
                Task {
                    id: 1,
                    completed: false,
                    task: "Make toast".to_string()
                }
            )
        }

        #[test]
        fn completed() {
            assert_eq!(
                Task::parse("1 [X] Make toast".to_string()),
                Task {
                    id: 1,
                    completed: true,
                    task: "Make toast".to_string()
                }
            )
        }

        #[test]
        fn malformed() {
            assert_eq!(
                Task::parse("1    [X]     Make toast".to_string()),
                Task {
                    id: 1,
                    completed: true,
                    task: "Make toast".to_string()
                }
            )
        }

        #[test]
        fn save() {
            let task = Task {
                id: 1,
                completed: true,
                task: "Eat eggs".to_string(),
            };
            assert_eq!(task.to_string(), "1 [X] Eat eggs")
        }
    }
}

use std::collections::HashMap;
use std::{io::{stdin, stdout, Write}, process};
use std::num::ParseIntError;
use crate::{clean_user_input, clear_screen};
use crate::item::Item;
use crate::task_list::TaskList;

#[derive(Debug)]
pub enum TaskInterfaceAction { List, Add, Update, Delete, Complete, Quit }

#[derive(Debug)]
pub struct TaskInterfaceActionMapItem {
    pub command: String,
    pub action: TaskInterfaceAction,
}

pub struct TaskInterface {
    pub task_list: TaskList,
    pub actions: HashMap<String, TaskInterfaceAction>,
}

impl TaskInterface {
    pub fn new(task_list: TaskList) -> TaskInterface {
        TaskInterface {
            task_list,
            actions: HashMap::new(),
        }
    }

    pub fn add_action(&mut self, item: TaskInterfaceActionMapItem) {
        self.actions.insert(item.command, item.action);
    }

    pub fn process_input(&mut self, mut command: String) {
        command = clean_user_input(command);

        if command.is_empty() {
            println!("Enter an action to perform");
            return;
        }

        match self.actions.get(&command) {
            Some(TaskInterfaceAction::List) => {
                self.print_tasks()
            },
            Some(TaskInterfaceAction::Add) => {
                Self::request_string_input(
                    String::from("Add a new task:"),
                    |task| {
                        self.task_list.add(task);
                    }
                );
            },
            Some(TaskInterfaceAction::Update) => {
                Self::request_id_input(
                    String::from("Enter the ID of the task to update:"),
                    |id| {
                        if !self.task_list.id_exists(id) {
                            println!("No task with an ID of {} exists", id);
                            return;
                        }

                        Self::request_string_input(
                            String::from("Enter the updated task:"),
                            |content| {
                                self.task_list.update(id, content);
                            }
                        );
                    }
                );
            },
            Some(TaskInterfaceAction::Complete) => {
                Self::request_id_input(
                    String::from("Enter the ID of the task to mark as complete:"),
                    |id| {
                        self.task_list.mark_as_complete(id);
                    }
                );
            },
            Some(TaskInterfaceAction::Delete) => {
                Self::request_id_input(
                    String::from("Enter the ID of the task to delete:"),
                    |id| {
                        if !self.task_list.id_exists(id) {
                            println!("No task with an ID of {} exists", id);
                            return;
                        }

                        self.task_list.delete(id);
                    }
                );
            },
            Some(TaskInterfaceAction::Quit) => {
                clear_screen();
                process::exit(0);
            },
            None => println!("No actions found matching \"{}\"", command),
        }
    }

    fn request_string_input(instruction: String, mut callback: impl FnMut(String) -> ()) {
        if !instruction.is_empty() {
            println!("{}", instruction);
        }

        print!("> ");
        stdout().flush().expect("Couldn't flush buffer");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Couldn't read input");
        input = clean_user_input(input);
        callback(input);
    }

    fn request_id_input(instruction: String, mut callback: impl FnMut(isize) -> ()) {
        if !instruction.is_empty() {
            println!("{}", instruction);
        }

        print!("> ");
        stdout().flush().expect("Couldn't flush buffer");

        Self::request_string_input(
            instruction,
            |input| {
                let id: Result<isize, ParseIntError> = input.parse::<isize>();

                match id {
                    Ok(safe_id) => callback(safe_id),
                    Err(_) => println!("An ID must be a number"),
                }
            }
        );
    }

    fn print_tasks(&mut self) {
        let tasks = self.task_list.get_tasks();

        if tasks.is_empty() {
            return;
        }

        for task in tasks {
            println!("{}", Self::format_task(task));
        }
    }

    fn format_task(task: &Item) -> String {
        let mut done: String = String::from("✗");

        if task.done() {
            done = String::from("✓");
        }

        return format!("{} {}. {}", done, task.id(), task.content());
    }
}
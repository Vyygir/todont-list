use crate::item::Item;

type TaskListItems = Vec<Item>;

#[derive(Debug)]
pub struct TaskList {
    tasks: TaskListItems
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new()
        }
    }

    pub fn get_tasks(&mut self) -> &TaskListItems {
        &self.tasks
    }

    pub fn add(&mut self, content: String) {
        let id: usize = self.get_next_id();
        let task = Item::new(id, String::from(content));

        self.tasks.push(task);
    }

    pub fn update(&mut self, id: usize, content: String) {
        self.find(id).unwrap().update(content);
    }

    pub fn delete(&mut self, id: usize) {
        match self.find_index(id) {
            Ok(index) => { self.tasks.remove(index); },
            Err(e) => println!("{}", e),
        };
    }

    pub fn mark_as_complete(&mut self, id: usize) {
        match self.find(id) {
            Ok(item) => item.mark_as_complete(),
            Err(e) => println!("{}", e),
        };
    }

    fn get_next_id(&mut self) -> usize {
        if self.tasks.is_empty() {
            return 1;
        }

        return self.tasks.last().unwrap().id() + 1;
    }

    fn find(&mut self, id: usize) -> Result<&mut Item, String> {
        let item = self.tasks.iter_mut().find(|item| { item.id() == id });

        match item {
            Some(item) => Ok(item),
            None => Err(format!("No task exists with an ID of {}", id)),
        }
    }

    fn find_index(&mut self, id: usize) -> Result<usize, String> {
        let item = self.tasks.iter().position(|item| { item.id() == id });

        match item {
            Some(index) => Ok(index),
            None => Err(format!("No task exists with an ID of {}", id)),
        }
    }

    pub fn id_exists(&mut self, id: usize) -> bool {
        self.find(id).is_ok()
    }
}
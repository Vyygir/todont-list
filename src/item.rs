#[derive(Debug)]
pub struct Item {
    id: usize,
    content: String,
    done: bool,
}

impl Item {
    pub fn new(id: usize, content: String) -> Item {
        Item {
            id,
            content,
            done: false,
        }
    }

    pub fn update(&mut self, content: String) {
        self.content = content;
    }

    pub fn mark_as_complete(&mut self) {
        self.done = true;
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn done(&self) -> bool {
        self.done
    }
}
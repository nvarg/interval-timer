#[derive(Debug)]
pub struct Queue<T> {
    items: Vec<T>,
    front_index: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            front_index: 0,
        }
    }

    pub fn get(&self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }

        self.items.get(self.front_index)
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.items.is_empty() {
            return None;
        }

        self.items.get_mut(self.front_index)
    }

    pub fn next(&mut self) {
        let front = self.items.get(self.front_index);
        if front.is_some() {
            self.front_index = (self.front_index + 1) % self.items.len()
        }
    }

    pub fn set(&mut self, values: Vec<T>) {
        self.items = values.into_iter().collect();
    }

    pub fn is_last(&self) -> bool {
        self.front_index == self.items.len() - 1
    }
}

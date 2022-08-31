use super::random::Random;

pub struct Queue<T: Clone, TRandom: Random<T>> {
    random: TRandom,
    size: usize,
    next_items: Vec<T>,
}

impl<T: Clone, TRandom: Random<T>> Queue<T, TRandom> {
    pub fn new(size: usize, random: TRandom) -> Self {
        Self {
            random,
            size,
            next_items: Vec::new(),
        }
    }

    pub fn next(&mut self) -> T {
        self.populate_next_items();
        let item = self.next_items.remove(0);
        self.populate_next_items();
        item
    }

    pub fn next_items(&mut self) -> &Vec<T> {
        self.populate_next_items();
        &self.next_items
    }

    fn populate_next_items(&mut self) {
        while self.next_items.len() < self.size {
            self.next_items.push(self.random.next())
        }
    }
}

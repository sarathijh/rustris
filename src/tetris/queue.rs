use super::random::Random;

pub struct Queue<T: Clone, TRandom: Random<T>> {
    random: TRandom,
    size: usize,
    next_items: Vec<T>,
}

impl<T: Clone, TRandom: Random<T>> Queue<T, TRandom> {
    pub fn new(size: usize, mut random: TRandom) -> Self {
        Self {
            random,
            size,
            next_items: Vec::new(),
        }
    }

    pub fn next(&mut self) -> T {
        self.random.next()
    }

    pub fn next_items(&mut self) -> &Vec<T> {
        while self.next_items.is_empty() {
            self.next_items.push(self.random.next())
        }
        &self.next_items
    }
}

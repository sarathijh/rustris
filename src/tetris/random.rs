use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Random<T: Clone> {
    fn next(&mut self) -> T;
}

pub struct RandomBag<T: Clone> {
    original_values: Vec<T>,
    current_values: Vec<T>,
}

impl<T: Clone> RandomBag<T> {
    pub fn new(values: Vec<T>) -> Self {
        let mut current_values = values.to_vec();
        current_values.shuffle(&mut thread_rng());
        Self {
            original_values: values,
            current_values,
        }
    }
}

impl<T: Clone> Random<T> for RandomBag<T> {
    fn next(&mut self) -> T {
        if let Some(value) = self.current_values.pop() {
            value
        } else {
            self.current_values = self.original_values.to_vec();
            self.current_values.shuffle(&mut thread_rng());
            self.next()
        }
    }
}

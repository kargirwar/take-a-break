mod types {
    #[derive(Debug)]
    pub struct WrappingVec<T> {
        data: Vec<T>,
        current_index: usize,
    }

    impl<T> WrappingVec<T> {
        pub fn new() -> Self {
            WrappingVec {
                data: Vec::new(),
                current_index: 0,
            }
        }

        pub fn push(&mut self, item: T) {
            self.data.push(item);
        }

        pub fn clear(&mut self) {
            self.data.clear();
            self.current_index = 0;
        }

        pub fn next(&mut self) -> Option<&T> {
            if self.data.is_empty() {
                None
            } else {
                let current_item = &self.data[self.current_index];
                self.current_index = (self.current_index + 1) % self.data.len();
                Some(current_item)
            }
        }

        pub fn prev(&self) -> Option<&T> {
            let num_elements = self.data.len();
            if num_elements == 0 {
                return None;
            }

            let prev_index = if self.current_index == 0 {
                num_elements - 1
            } else {
                self.current_index - 1
            };

            self.at(prev_index)
        }

        pub fn at(&self, index: usize) -> Option<&T> {
            if index < self.data.len() {
                Some(&self.data[index])
            } else {
                None
            }
        }

        pub fn sort_by_key<F>(&mut self, key_extractor: F)
        where
            F: FnMut(&T) -> u64,
        {
            self.data.sort_by_key(key_extractor);
        }
    }
}

pub use types::*;

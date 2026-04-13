

pub trait ExtVec<T> {
    fn with(self, item: T) -> Self;
    fn chain<I: IntoIterator<Item = T>>(self, iter: I) -> Self; 
}

impl<T> ExtVec<T> for Vec<T> {
    /// Consumes the vec, pushes an item, and returns the vec.
    fn with(mut self, item: T) -> Self {
        self.push(item);
        self 
    }

    /// Consumes the vec, extends it with an iterator, and returns the vec.
    fn chain<I: IntoIterator<Item = T>>(mut self, iter: I) -> Self {
        self.extend(iter);
        self
    }
}
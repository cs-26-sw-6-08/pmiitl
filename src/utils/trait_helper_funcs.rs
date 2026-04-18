use std::error::Error;

use crate::errors;



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


pub trait OptionExt<T> {
    fn or_pop_err(self) -> Result<T, Box<dyn Error>>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn or_pop_err(self) -> Result<T, Box<dyn Error>> {
        self.ok_or(errors::Error::ValueStackPopError.into())
    }
}
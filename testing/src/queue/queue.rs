//! Generic queue interface.

use std::fmt::Debug;

/// A generic queue interface.
pub trait Queue<T: Debug> {
    /// Add an element to the queue.
    fn add(&mut self, elem: T);

    /// Get an element from the queue.
    fn get(&mut self) -> Option<T>;

    /// Get the length of the queue.
    fn len(&self) -> usize;

    /// Check if the queue is empty.
    fn is_empty(&self) -> bool;

    /// Dump the contents of the queue into another queue.
    fn dump(&mut self, other: &mut dyn Queue<T>);

    /// Get the next element in the queue without removing it.
    fn peek(&self) -> Option<&T>;
}

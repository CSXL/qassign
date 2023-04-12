//! Queue implementation using a linked list.
use super::Queue;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ptr;

/// A queue implemented using a linked list.
/// The queue is FIFO (first in, first out).
pub struct FIFOQueue<T: Debug> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> FIFOQueue<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        FIFOQueue {
            head: None,
            tail: ptr::null_mut(),
        }
    }
}

impl<T> Queue<T> for FIFOQueue<T>
where
    T: Debug,
{
    /// Add an element to the back of the queue.
    /// This operation should compute in O(1) time.
    fn add(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            // Not empty, update the existing tail node.
            // Wrapped in unsafe because we are dereferencing a raw pointer.
            unsafe { (*self.tail).next = Some(new_tail) };
        } else {
            // Empty queue, update the head.
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    /// Remove an element from the front of the queue.
    /// This operation should compute in O(1) time.
    fn get(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            if node.next.is_none() {
                // Empty queue, update the tail.
                self.tail = ptr::null_mut();
            }
            self.head = node.next;
            node.elem
        })
    }

    /// Return a reference to the next element in the queue without removing it.
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns the number of elements in the queue.
    fn len(&self) -> usize {
        let mut len = 0;
        let mut cur_link = &self.head;
        while let &Some(ref node) = cur_link {
            len += 1;
            cur_link = &node.next;
        }
        len
    }

    /// Return true if the queue contains no elements.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Dumps the contents of the queue to another queue.
    // The elements are in the same order as they would be removed from the queue.
    fn dump(&mut self, other: &mut dyn Queue<T>) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            other.add(boxed_node.elem);
            cur_link = boxed_node.next.take();
        }
        self.tail = ptr::null_mut();
    }
}

impl<T> Drop for FIFOQueue<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Display for FIFOQueue<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut cur_link = &self.head;
        write!(f, "FIFOQueue<{}> [", std::any::type_name::<T>())?;
        while let &Some(ref node) = cur_link {
            write!(f, "{}", node.elem)?;
            // Write w/ a delimiter if there are more elements.
            if node.next.is_some() {
                write!(f, ", ")?;
            }
            cur_link = &node.next;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = FIFOQueue::new();
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());

        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(q.len(), 3);
        assert!(!q.is_empty());

        assert_eq!(q.get(), Some(1));
        assert_eq!(q.get(), Some(2));
        assert_eq!(q.get(), Some(3));
        assert_eq!(q.get(), None);
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());

        q.add(4);
        q.add(5);
        q.add(6);
        assert_eq!(q.len(), 3);
        assert!(!q.is_empty());

        let mut q2 = FIFOQueue::new();
        q.dump(&mut q2);
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());
        assert_eq!(q2.len(), 3);
        assert!(!q2.is_empty());
        assert_eq!(q2.get(), Some(4));
        assert_eq!(q2.get(), Some(5));
        assert_eq!(q2.get(), Some(6));
        assert_eq!(q2.get(), None);
        assert_eq!(q2.len(), 0);
        assert!(q2.is_empty());
    }

    #[test]
    fn test_queue_display() {
        let mut q = FIFOQueue::new();
        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(format!("{}", q), "FIFOQueue<i32> [1, 2, 3]");
    }

    #[test]
    fn test_queue_drop() {
        let mut q = FIFOQueue::new();
        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(q.len(), 3);
        assert!(!q.is_empty());
        drop(q);
    }

    #[test]
    fn test_queue_drop_empty() {
        let q: FIFOQueue<i32> = FIFOQueue::new();
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());
        drop(q);
    }

    #[test]
    fn test_queue_dump() {
        let mut q = FIFOQueue::new();
        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(q.len(), 3);
        assert!(!q.is_empty());

        let mut q2 = FIFOQueue::new();
        q.dump(&mut q2);
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());
        assert_eq!(q2.len(), 3);
        assert!(!q2.is_empty());
        assert_eq!(q2.get(), Some(1));
        assert_eq!(q2.get(), Some(2));
        assert_eq!(q2.get(), Some(3));
        assert_eq!(q2.get(), None);
        assert_eq!(q2.len(), 0);
        assert!(q2.is_empty());
    }

    #[test]
    fn test_queue_peek() {
        let mut q = FIFOQueue::new();
        assert_eq!(q.peek(), None);
        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.get(), Some(1));
        assert_eq!(q.peek(), Some(&2));
        assert_eq!(q.peek(), Some(&2));
        assert_eq!(q.get(), Some(2));
        assert_eq!(q.peek(), Some(&3));
        assert_eq!(q.peek(), Some(&3));
        assert_eq!(q.get(), Some(3));
        assert_eq!(q.peek(), None);
        assert_eq!(q.peek(), None);
        assert_eq!(q.get(), None);
    }
}

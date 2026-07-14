//! The base definition of the `LinkedList` with utility functions like `len`
//! or `is_empty`.

use crate::node::Node;
use std::ptr::NonNull;

/// My implementation of a `LinkedList` in Rust.
#[derive(Clone, Debug, Default)]
pub struct LinkedList<T> {
    /// The head of the list, is `None` iff the list is empty.
    pub(crate) head: Option<Box<Node<T>>>,
    /// A pointer to the tail of the list
    pub(crate) tail: Option<NonNull<Node<T>>>,
    /// The amount of values stored in the list.
    pub(crate) length: usize,
}

impl<T> LinkedList<T> {
    /// Creates a new `LinkedList`.
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Returns the amount of elements in the list
    pub fn len(&self) -> usize {
        self.length
    }

    /// Whether or not there are any values stored in the list.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Clears the list of any and all elements within.
    pub fn clear(&mut self) {
        *self = LinkedList::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<usize> = LinkedList::new();
        assert_eq!(list.length, 0, "new list is not empty");
        assert_eq!(list.head, None, "new list already has a head");
        assert_eq!(list.tail, None, "new list already has a tail");
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0, "new list is not empty");
        for index in 1..100 {
            list.push_front(index);
            assert_eq!(list.len(), index, "List length does not increase correctly");
        }
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        for index in 0..10 {
            list.push_front(index);
            assert!(!list.is_empty());
        }

        assert!(!list.is_empty());
        for _ in 0..10 {
            assert!(!list.is_empty());
            list.pop_front();
        }

        assert!(list.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();
        list.push_front(1 as i32);
        list.clear();
        assert_eq!(list.length, 0, "Length was not reset");
        assert_eq!(list.head, None, "Head was not reset");
        assert_eq!(list.tail, None, "Tail was not reset");
    }
}

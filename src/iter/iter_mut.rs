//! This module defines code for converting `LinkedList`s into and from
//! iterators.

use crate::list::LinkedList;
use crate::node::Node;

pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    /// Creates an iterator over mutable references in the list.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> std::iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.take()?;
        self.current = node.next.as_deref_mut();
        Some(&mut node.value)
    }
}

#[cfg(test)]
mod test_linked_list_iter {
    use super::*;

    #[test]
    fn test_iter_mut() {
        let vec = vec![0, 1, 2, 3, 4];
        let mut list: LinkedList<_>;
        list = vec.clone().into_iter().collect();
        for value in list.iter_mut() {
            *value *= 2;
        }

        for (index, value) in list.iter().enumerate() {
            assert_eq!(index * 2, *value);
        }
    }
}

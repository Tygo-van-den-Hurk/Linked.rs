//! This module defines the methods to pop, push, or observe elements from the
//! back of the `LinkedList`.

use crate::list::LinkedList;
use crate::node::Node;
use std::ptr::NonNull;

impl<T> LinkedList<T> {
    /// Appends an element to the back of the list.
    pub fn push_back(&mut self, value: T) {
        self.length += 1;

        // Create new node to replace the old tail
        let previous = self.tail.take();
        let next = None;
        let node = Node {
            previous,
            next,
            value,
        };

        let mut new_tail = Box::new(node);
        let pointer = NonNull::from(&mut *new_tail);

        // Updating the tail
        self.tail = Some(pointer);

        // Glue the node to the list
        match &mut new_tail.previous.clone() {
            None => self.head = Some(new_tail),
            Some(old_tail) => unsafe {
                old_tail.as_mut().next = Some(new_tail);
            },
        }
    }

    /// Grabs the node at the right end of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // Since it's not empty we will be taking something.
        self.length -= 1;

        // of there was only one value, then just take the head
        if self.length == 0 {
            let node = self.head.take().unwrap();
            self.tail = None;
            return Some(node.value);
        }

        let old_tail_pointer = self.tail.take().unwrap();
        let old_tail = unsafe { old_tail_pointer.as_ref() };
        let mut new_tail_pointer = old_tail.previous.unwrap();
        let new_tail = unsafe { new_tail_pointer.as_mut() };

        self.tail = Some(NonNull::from(&mut *new_tail));
        let result = new_tail.next.take().unwrap();
        Some(result.value)
    }

    /// Allows you to get a non-mutable borrow to the back most element.
    pub fn back(&self) -> Option<&T> {
        match &self.tail {
            None => None,
            Some(pointer) => unsafe {
                let node = pointer.as_ref();
                Some(&node.value)
            },
        }
    }

    /// Allows you to get a mutable borrow to the back most element.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        match &mut self.tail {
            None => None,
            Some(pointer) => unsafe {
                let node = pointer.as_mut();
                Some(&mut node.value)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_base_case() {
        let mut list = LinkedList::new();
        let value = 123;

        list.push_back(value);
        assert_eq!(list.length, 1);

        match list.head {
            None => panic!("head was not updated"),
            Some(node) => assert_eq!(node.value, value, "inner value mutated"),
        }

        match list.tail {
            None => panic!("tail was not updated"),
            Some(pointer) => unsafe {
                let node = pointer.as_ref();
                assert_eq!(node.value, value, "inner value mutated")
            },
        }
    }

    #[test]
    fn test_pop_back_base_case() {
        let mut list = LinkedList::new();
        let tail_value = 123;
        list.push_back(tail_value);

        match list.pop_back() {
            None => panic!("Value was lost"),
            Some(value) => assert_eq!(value, tail_value, "tail value mutated"),
        }

        assert_eq!(list.length, 0, "length was not reset");
        assert_eq!(list.head, None, "head was not reset");
        assert_eq!(list.tail, None, "tail was not reset");
    }

    #[test]
    fn test_pop_back_step_case() {
        let mut list = LinkedList::new();
        let tail_value = 123;
        list.push_back(tail_value);
        for value in 2..99 {
            list.push_back(value);
        }

        for index in (2..99).rev() {
            match list.pop_back() {
                None => panic!("Value was lost"),
                Some(value) => assert_eq!(value, index, "inner value mutated"),
            }

            assert_eq!(list.length, index - 1, "length did not shrink linearly");
        }

        match list.pop_back() {
            None => panic!("Value was lost"),
            Some(value) => assert_eq!(value, tail_value, "inner value mutated"),
        }

        assert_eq!(list.length, 0, "length was not reset");
        assert_eq!(list.head, None, "head was not reset");
        assert_eq!(list.tail, None, "tail was not reset");
    }

    #[test]
    fn test_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.back(), None);
        list.push_back(1);
        assert_eq!(list.back(), Some(&1));
        list.push_back(2);
        assert_eq!(list.back(), Some(&2));
        list.pop_back();
        assert_eq!(list.back(), Some(&1));
        list.pop_back();
        assert_eq!(list.back(), None);
    }

    #[test]
    fn test_back_mut() {
        let mut list = LinkedList::new();
        assert_eq!(list.back_mut(), None);
        list.push_back(1);
        assert_eq!(list.back_mut(), Some(&mut 1));
        list.push_back(2);
        assert_eq!(list.back_mut(), Some(&mut 2));
        list.pop_back();
        assert_eq!(list.back_mut(), Some(&mut 1));
        list.pop_back();
        assert_eq!(list.back_mut(), None);
    }
}

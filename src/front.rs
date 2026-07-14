//! This module defines the methods to pop, push, or observe elements from the
//! front of the `LinkedList`.

use crate::list::LinkedList;
use crate::node::Node;
use std::ptr::NonNull;

impl<T> LinkedList<T> {
    /// Pops the front most node from the list and returns its value.
    pub fn push_front(&mut self, value: T) {
        self.length += 1;

        // Create new node
        let previous = None;
        let next = self.head.take();
        let node = Node {
            previous,
            next,
            value,
        };
        let mut new_head = Box::new(node);
        let pointer = NonNull::from(&mut *new_head);

        // Updating the old head
        if let Some(old_head) = &mut new_head.next {
            old_head.previous = Some(pointer);
        }

        // Initialize tail if the list was empty before
        if self.tail.is_none() {
            self.tail = Some(pointer);
        }

        self.head = Some(new_head);
    }

    /// Pops the front most node from the list and returns its value.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.length -= 1;
                if self.length == 0 {
                    self.tail = None;
                }

                self.head = node.next;
                if let Some(new_head) = &mut self.head {
                    new_head.previous = None;
                }

                Some(node.value)
            }
        }
    }

    /// Allows you to get a non-mutable borrow to the front most element.
    pub fn front(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.value),
        }
    }

    /// Allows you to get a mutable borrow to the front most element.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            None => None,
            Some(node) => Some(&mut node.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_front_base_case() {
        let mut list = LinkedList::new();
        let value = 123;

        list.push_front(value);
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
    fn pop_front_base_case() {
        let mut list = LinkedList::new();
        let value = 123;
        list.push_front(value);
        assert_eq!(list.pop_front(), Some(value), "inner value mutated");

        assert_eq!(list.length, 0, "length was not reset");
        assert_eq!(list.head, None, "head was not reset");
        assert_eq!(list.tail, None, "tail was not reset");
    }

    #[test]
    fn push_front_step_case() {
        let mut list = LinkedList::new();
        let tail_value = 123;

        list.push_front(tail_value);
        for value in 2..99 {
            list.push_front(value);

            assert_eq!(list.length, value, "list length does not grow linearly");

            match list.head {
                None => panic!("head was not updated"),
                Some(ref node) => assert_eq!(node.value, value, "inner value mutated"),
            }

            match list.tail {
                None => panic!("tail was not updated"),
                Some(pointer) => unsafe {
                    let node = pointer.as_ref();
                    assert_eq!(node.value, tail_value, "tail mutated");
                },
            }
        }
    }

    #[test]
    fn pop_front_step_case() {
        let mut list = LinkedList::new();
        let tail_value = 123;

        list.push_front(tail_value);
        for value in 1..99 {
            list.push_front(value);
        }

        for index in (1..99).rev() {
            match list.pop_front() {
                None => panic!("inner value was lost"),
                Some(value) => assert_eq!(value, index, "inner value mutated"),
            }

            assert_eq!(list.length, index, "length did not shrink linearly");
        }

        match list.pop_front() {
            None => panic!("tail value was lost"),
            Some(value) => {
                assert_eq!(value, tail_value, "tail value mutated")
            }
        }

        assert_eq!(list.length, 0, "length was not reset");
        assert_eq!(list.head, None, "head was not reset");
        assert_eq!(list.tail, None, "tail was not reset");
    }

    #[test]
    fn test_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.front(), None);
        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
        list.push_front(2);
        assert_eq!(list.front(), Some(&2));
        list.pop_front();
        assert_eq!(list.front(), Some(&1));
        list.pop_front();
        assert_eq!(list.front(), None);
    }

    #[test]
    fn test_front_mut() {
        let mut list = LinkedList::new();
        assert_eq!(list.front_mut(), None);
        list.push_front(1);
        assert_eq!(list.front_mut(), Some(&mut 1));
        list.push_front(2);
        assert_eq!(list.front_mut(), Some(&mut 2));
        list.pop_front();
        assert_eq!(list.front_mut(), Some(&mut 1));
        list.pop_front();
        assert_eq!(list.front_mut(), None);
    }
}

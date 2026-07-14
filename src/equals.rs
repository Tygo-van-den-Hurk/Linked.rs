//! This module implements functions to check equality between two linked
//! lists using the `core::cmp::Eq` and `core::cmp::PartialEq` traits.

use crate::list::LinkedList;
use crate::node::Node;

impl<T: Eq> core::cmp::Eq for Node<T> {}
impl<T: PartialEq> core::cmp::PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> core::cmp::Eq for LinkedList<T> {}
impl<T: PartialEq> core::cmp::PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut option1 = self.head.as_ref();
        let mut option2 = other.head.as_ref();
        while let (Some(node1), Some(node2)) = (option1, option2) {
            if node1.value != node2.value {
                return false;
            }

            option1 = node1.next.as_ref();
            option2 = node2.next.as_ref();
        }

        option1.is_none() && option2.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_empty_list() {
        assert_eq!(LinkedList::<usize>::new(), LinkedList::<usize>::new(),);
    }

    #[test]
    fn test_equality_list() {
        let mut list1 = LinkedList::new();
        list1.push_front(1);
        list1.push_front(2);
        list1.push_front(3);

        let mut list2 = LinkedList::new();
        list2.push_front(1);
        list2.push_front(2);
        list2.push_front(3);

        assert_eq!(list1, list2);
    }

    #[test]
    fn test_inequality_content_list() {
        let mut list1 = LinkedList::new();
        list1.push_front(1);
        list1.push_front(2);
        list1.push_front(3);

        let mut list2 = LinkedList::new();
        list2.push_front(0);
        list2.push_front(2);
        list2.push_front(3);

        assert_ne!(list1, list2);
    }

    #[test]
    fn test_inequality_length_list() {
        let mut list1 = LinkedList::new();
        list1.push_front(1);
        list1.push_front(2);
        list1.push_front(3);

        let mut list2 = LinkedList::new();
        list2.push_front(1);
        list2.push_front(2);
        list2.push_front(3);
        list2.push_front(4);

        assert_ne!(list1, list2);

        let mut list1 = LinkedList::new();
        list1.push_front(1);
        list1.push_front(2);
        list1.push_front(3);

        let mut list2 = LinkedList::new();
        list2.push_front(0);
        list2.push_front(1);
        list2.push_front(2);
        list2.push_front(3);

        assert_ne!(list1, list2);
    }
}

/// A single node in the `LinkedList`.
#[derive(Clone, Debug, Default)]
pub(crate) struct LinkedListNode<T> {
    /// The value of the node.
    pub(crate) value: T,
    /// The next node in the chain
    pub(crate) next: Option<Box<LinkedListNode<T>>>,
}

/// My implementation of a `LinkedList` in Rust.
#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    /// The head of the list, is `None` iff the list is empty.
    pub(crate) head: Option<Box<LinkedListNode<T>>>,
    /// The amount of values stored in the list.
    pub(crate) length: usize,
}

impl<T> LinkedList<T> {
    /// Creates a new `LinkedList`.
    pub fn new() -> Self {
        LinkedList {
            head: None,
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

    /// Pops the front most node from the list and returns its value.
    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        let node = LinkedListNode { value, next };
        self.head = Some(Box::new(node));
        self.length += 1;
    }

    /// Pops the front most node from the list and returns its value.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.length -= 1;
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

#[cfg(test)]
mod test_linked_list {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<usize> = LinkedList::new();
        assert!(list.len() == 0, "new list is not empty");
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert!(list.len() == 0, "new list is not empty");
        assert!(list.is_empty());

        for index in 0..100 {
            list.push_front(index);
            assert_eq!(list.len(), index + 1);
        }
    }

    #[test]
    fn test_push_and_pop_front() {
        let mut list = LinkedList::new();
        assert!(list.len() == 0, "new list is not empty");
        assert!(list.is_empty());
        assert!(list.pop_front() == None);

        let pushed = 1;
        list.push_front(pushed);
        assert!(list.len() == 1, "list does not grow");
        assert!(!list.is_empty());

        let popped = list.pop_front().unwrap();
        assert!(list.len() == 0, "list does not shrink");
        assert!(list.pop_front() == None);
        assert!(list.is_empty());
        assert!(popped == pushed,
            "what came out is not the same as what went in"
        );
    }
}

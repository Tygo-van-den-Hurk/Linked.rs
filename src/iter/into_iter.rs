use crate::list::LinkedList;
use crate::node::Node;

pub struct Iter<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> std::iter::Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.take()?;
        self.current = node.next;
        Some(node.value)
    }
}

impl<T> std::iter::IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: self.head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter() {
        let vec = vec![1, 2, 3, 4, 5];
        let list: LinkedList<_>;
        list = vec.clone().into_iter().collect();
        assert_eq!(
            list.into_iter().collect::<Vec<usize>>(),
            vec![1, 2, 3, 4, 5]
        );
    }
}

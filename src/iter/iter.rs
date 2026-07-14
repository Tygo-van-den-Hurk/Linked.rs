use crate::list::LinkedList;
use crate::node::Node;

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let vec = vec![0, 1, 2, 3, 4];
        let list: LinkedList<_>;
        list = vec.clone().into_iter().collect();
        let iter = list.iter();
        for (index, value) in iter.enumerate() {
            assert_eq!(&index, value);
        }
    }
}

use crate::list::LinkedList;

impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<R: IntoIterator<Item = T>>(iter: R) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_back(item);
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter() {
        let mut list: LinkedList<_>;
        list = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);
    }
}

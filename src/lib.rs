mod back;
mod equals;
mod front;
mod iter;
pub mod list;
mod node;

pub use list::LinkedList;

#[cfg(test)]
mod test_linked_list_iter {
    use super::*;

    #[test]
    fn test_end_to_end() {
        let mut list = LinkedList::new();
        let numbers = (0..5)
            .map(|n| n * 2)
            .fold(Vec::with_capacity(10), |mut all, n| {
                all.push((n, n + 1));
                all
            });

        for number in numbers {
            list.push_front(number.0);
            list.push_back(number.1);
        }

        assert_eq!(
            list,
            vec![8, 6, 4, 2, 0, 1, 3, 5, 7, 9].into_iter().collect(),
            "The order was not as expected"
        );

        list.pop_back();
        list.pop_back();
        list.pop_back();

        match list.tail {
            None => panic!("tail was list"),
            Some(pointer) => unsafe {
                let node = pointer.as_ref();
                assert_eq!(node.value, 3, "tail got corrupted over time");
                assert_eq!(node.next, None, "node.next was not removed correctly");
            },
        }

        assert_eq!(list.len(), 10 - 3, "length was not kept up to date");

        assert_eq!(list.front(), Some(&8), "front was affected");

        let back = list.pop_front().unwrap();
        list.push_back(back);

        assert_eq!(list.front(), Some(&6), "front was not kept up to date");

        assert_eq!(list.back(), Some(&8), "back was not kept up to date");
    }
}

use std::ptr::NonNull;

/// A single node in the `LinkedList`.
#[derive(Clone, Debug, Default)]
pub(crate) struct Node<T> {
    /// The value of the node.
    pub(crate) value: T,
    /// The next node in the chain
    pub(crate) next: Option<Box<Node<T>>>,
    /// The previous node in the chain
    pub(crate) previous: Option<NonNull<Node<T>>>,
}

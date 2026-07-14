# Linked.rs

This repository is just me practicing my Rust skills. Making a linked list in
Rust is apparently really hard. So I want to attempt making one from scratch
myself. I know that the standard library has one already implemented, however
if I used that one, I wouldn't learn anything.

I'm happy to announce that after a couple of hours: I did it! I implemented the
entire data structure without help myself in Rust. This is good progress from
when I was struggling with printing to the console because of issues with the 
borrow checker.

If you wanna read the process, keep reading. :D

## The Process

I started out with what I did know. I obviously knew what a linked list was,
and how in most languages they're build. From what I heard was the problem
was that just like in C, structs need a fixed size that is known at compile
time. So I started out with a node, and a value in a box:

```Rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
```

This allowed us to store both a value, and the next node. This would work in
and of itself. But the ergonomics are a little bad. So I fixed that by adding
a struct to manage it all under the hood:

```Rust
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {

    pub fn new();

    pub fn len();
    pub fn is_empty();

    pub fn push_front(&mut self, value: T);
    pub fn pop_front(&mut self, value: T) -> Option<T>;

    pub fn front(&self) -> Option<&T>;
    pub fn front_mut(&mut self) -> Option<&mut T>;
}
```

This works! However it reminds me more like a stack than a linked list though.
You can't look in the middle, you can't see the end either, etc. So, I needed
to fix that. This, was by far the hardest part.

I haven't written that much rust. I try doing [my LeetCode][leet], and Advent
of Code in Rust to learn the language. But that is it. So I haven't written
that much yet. The pain that is managing pointers can only be described as
"less painful" then C, but painful enough to quit on a bad day. 

To implement access to the back of the list, we have to make both structs able
to traverse both directions like so:

```Rust
struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    ...
}

pub struct LinkedList<T> {
    tail: Option<NonNull<Node<T>>>,
    ...
}
```

What is `NonNull` you might ask? No idea, I'm as clueless as you are right now.
From what I understand, it is a reference to non-null memory. So, a valid
pointer to something. Creating them isn't hard:

```Rust
NonNull::from(&mut *value);
```

The problem is using them. Because the borrow checker cannot check wether or
not we were super boosted and wrote it all wrong, we have to use the `unsafe`
keyword. That alone is scary enough to be like "ye, no thanks". However I think
this is kind of the only way of doing this in rust. So for example to access
the back value:

```Rust
impl<T> LinkedList<T> {
    pub fn back(&self) -> Option<&T> {
        match &self.tail {
            None => None,
            Some(pointer) => unsafe {
                let node = pointer.as_ref();
                Some(&node.value)
            }
        }
    }
}
```

The problem isn't owning a pointer, the problem is getting the value behind it.
After you figure out how they work though, it's not that scary anymore. To get
it right I build the tests in a way that reminds me of the functional 
programming course I took this year.

You "prove" the validity of the function as an induction proof. You first test
the base case: adding and removing 1 element, and then test for 100 insertions
and removals. After 100 I call it safe enough.

Now our linked list API looks not to bad if I do say so myself. We can insert
and remove elements from the front and back, as well get references to them.
You can then add another couple of helpers like equality:

```Rust
impl<T: Eq> core::cmp::Eq for LinkedList<T> {}
impl<T: PartialEq> core::cmp::PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        ...
    }
}
```

Or another one, one that is far more useful is the `Iterator` trait. This
allows you to loop over them, map, fold, reduce, filter, and much more, good
stuff. Getting the `FromIterator` trait working isn't that hard, after all,
all you need to do, is consume all and append to the list:

```Rust
impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<R: IntoIterator<Item = T>>(iter: R) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_back(item);
        }

        list
    }
}
```

The `Iterator` trait itself, and `IntoIterator` are far more confusing to me.
You don't implement the `Iterator` trait on your data structure, but rather
you implement a function returning a struct that implements the `iterator`
trait. That was a lot more work than I thought it would be, and also far more
repetitive than feels acceptable?

For example:

```Rust
//! into_iter.rs
pub struct Iter<T> { ... }
impl<T> std::iter::Iterator for Iter<T> { ... }
impl<T> std::iter::IntoIterator for LinkedList<T> { ... }

//! iter_mut.rs
pub struct IterMut<'a, T> { ... }
impl<'a, T> std::iter::Iterator for IterMut<'a, T> { ... }
impl<T> LinkedList<T> { ... }

//! iter_ref.rs
pub struct Iter<'a, T> { ... }
impl<'a, T> std::iter::Iterator for Iter<'a, T> { ... }
impl<T> LinkedList<T> { ... }
```

I feel like that should have been at least a little bit reusable. Maybe it is
and I am just too new to see it, but with references etc this feels
non-reuseable.

Now that I've implemented the `iterator` trait, I think I've done all I want
to. There is probably more I can come up with, but by all accounts all major
tasks are done.

## License

All code in this repository falls under a [license][LICENSE]

[leet]: https://redirects.tygo.van.den.hurk.dev/leetcode/
[LICENSE]: ./LICENSE

use std::{iter::FromIterator, marker::PhantomData, ptr::NonNull};
// Mostly copied from rust src code for studying purpose
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T:'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

#[derive(Debug, Clone)]
pub struct IterMut<'a, T:'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,   
    marker: PhantomData<&'a Node<T>>,
}

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    list: LinkedList<T> 
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {next: None, prev: None, element}
    }
    fn into_element(self) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        LinkedList {head: None, tail: None, len: 0}
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {head: self.head, tail: self.tail, len: self.len, marker: PhantomData}
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {head: self.head, tail: self.tail, len: self.len, marker:PhantomData}
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn clear(&mut self) {
        *self = Self::new()
    }
    pub fn contains(&self, x: &T) -> bool
    where T: PartialEq<T> {
        self.iter().any(|e| e == x)
    }
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node_ptr| &node_ptr.as_ref().element)
        }
    }
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node_ptr| &mut node_ptr.as_mut().element)
        }
    }
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail.as_ref().map(|node_ptr| &node_ptr.as_ref().element)
        }
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail.as_mut().map(|node_ptr| &mut node_ptr.as_mut().element)
        }
    }
    pub fn push_front(&mut self, elt: T) {
        unsafe {
            let mut node = Box::new(Node::new(elt));
            node.next = self.head;
            let onn = NonNull::new(Box::leak(node));
            
            // handles linking in the reverse direction
            match self.head {
                None => self.tail = onn,
                Some(head) => (*head.as_ptr()).prev = onn,
            }

            self.head = onn;
            self.len += 1;
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }
    pub fn push_back(&mut self, elt: T) {
        todo!()
    }
    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }
    pub fn append(&mut self, other: &mut Self) {
        todo!()
    }
    pub fn remove(&mut self, at: usize) -> T {
        todo!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    type Item = &'a T;
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.next;
                &node.element
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &mut node.element
            })
        }
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.tail = node.next;
                &mut node.element
            })
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}


impl<T> FromIterator for LinkedList<T> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut list =  Self::new();
        list.extend(iter);
        list
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {list: self}
    }
}
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Extend for LinkedList<T> {
    fn extend<T: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
    fn extend_one(&mut self, item: T) {
        todo!()
    }
}
impl<'a, T: 'a + Copy> Extend<&'a T> for LinkedList<T> {
    fn extend<T: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
    fn extend_one(&mut self, &item: &'a T) {
        self.push_back(elem);
    }
}



fn main() {
    let mut tst: LinkedList<i32> = LinkedList::new();
    tst.push_back(10);
    tst.push_back(15);
    for n in tst.iter() {
        println!("{}", n);
    }
    println!("{:?}", 2);
}

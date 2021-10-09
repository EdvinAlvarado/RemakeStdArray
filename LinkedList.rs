trait Collectable<T: std::marker::Copy> {
    fn push(&mut self, n: T);
    fn pop(&mut self) -> Option<T>;
    fn get(&self, i: usize) -> Option<T>;
}

type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node<T> {
    pub val: T,
    pub next: NodeBox<T>,
}

impl<T: std::marker::Copy> Node<T> {
    pub fn new(n: T) -> Node<T> {
        Node {val: n, next: None,}
    }
    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }
    fn box_new(n: T) -> NodeBox<T> {
        Node::boxer(Node::new(n))
    }
    pub fn first(& self) -> T {self.val}
    pub fn last(&mut self) -> T { //last() is already taken 
        match self.next {
            None => self.val,
            Some(ref mut next) => next.end(),
        }
    }
}

impl<T: std::marker::Copy> Collectable<T> for Node<T> {
    fn push(&mut self, n: T) {
        match self.next {
            None => {self.next = Node::box_new(n);},
            Some(ref mut next) => {next.push(n);},
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.next.is_none() {
            None
        } else {
            let mut last_node_ptr = &mut **self.next.as_mut().unwrap() as *mut Node<T>;
            let mut last_node_ptr_before = last_node_ptr;

            unsafe {
                while (*last_node_ptr).next.is_some() {
                    last_node_ptr_before = last_node_ptr;
                    last_node_ptr = last_node_ptr as *mut Node<T>;
                }
                let res = (*last_node_ptr).val;
                (*last_node_ptr_before).next = None;
                Some(res)
            }
        }
    }
    fn get(&self, i:usize) -> Option<T> {
        match i {
            0 => Some(self.val),
            i if i > 0 => {
                match self.next {
                    None => None,
                    Some(ref box_node) => (*box_node).get(i-1)
                }
            },
            _ => None,
        }
    }
}
//--------------------------------------------------------------------------------
#[derive(Debug)]
struct LinkedList<T> {
    size: usize,
    pub head: Node<T>,
    // pub tail: Option<Box<Node<T>>>,
}

impl<T: std::marker::Copy> LinkedList<T> {
    pub fn new(n: T) -> LinkedList<T> {
        LinkedList {
            size: 1,
            head: Node::new(n),
            // tail: Some(start),
        }
    }
    pub fn len(&self) -> usize {self.size}
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            size: self.size,
            head: &self.head,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            size: self.size,
            head: &mut self.head,
        }
    }
}

impl<T: std::marker::Copy> Collectable<T> for LinkedList<T> {
     fn push(&mut self, n: T) {
        self.head.push(n);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        self.size -= 1;
        self.head.pop()
    }
    fn get(&self, i:usize) -> Option<T> {
        if i >= self.size {None} else {self.head.get(i)}
    }   
}

// Consumable LinkedList Iterator
struct IntoIter<T> {
    size: usize,
    head: Node<T>
}

impl<T: std::marker::Copy> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            size: self.size,
            head: self.head,
        }
    }
}

impl<T: std::marker::Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
         match self.size {
            0 => {
                None
            },
            _ => {
                let temp = self.head.val;
                self.size -= 1;
                match self.head.next.as_mut() {
                    Some(box_node) => {self.head = box_node.as_mut().clone();},
                    None => {();},
                }
                Some(temp)
            },
        }
    }
}

// Referenced LinkedList Iterator
struct Iter<'a, T: 'a> {
    size: usize,
    head: &'a Node<T>
}

impl<'a, T: std::marker::Copy> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: std::marker::Copy> Iterator for Iter<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
         match self.size {
            0 => {
                None
            },
            _ => {
                let temp = self.head.val;
                self.size -= 1;
                match self.head.next.as_mut() {
                    Some(box_node) => {self.head = box_node.as_mut().clone();},
                    None => {();},
                }
                Some(temp)
            },
        }
    }
}
// Referenced mut LinkedList Iterator
struct IterMut<'a, T: 'a> {
    size: usize,
    head: &'a mut Node<T>
}

impl<'a, T: std::marker::Copy> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

fn main() {
    let mut lst = LinkedList::<i32>::new(10);
    lst.push(20);
    println!("{:?}", lst);
    for n in &mut lst {
        println!("{:?}", n);
    }
    println!("{:?}", lst);
}

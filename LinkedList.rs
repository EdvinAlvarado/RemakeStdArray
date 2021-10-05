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
    pub fn push(&mut self, n: T) {
        match self.next {
            None => {self.next = Node::box_new(n);},
            Some(ref mut next) => {next.push(n);},
        }
    }
    // FIXME add boundary for size 1 node
    // FIXME add popping
    pub fn pop(&mut self) -> T {
        match self.next {
            None => self.val,
            Some(ref mut next) => next.pop(),
        }
    }
    pub fn first(& self) -> T {self.val}
    pub fn end(&mut self) -> T { //last() is already taken 
        match self.next {
            None => self.val,
            Some(ref mut next) => next.end(),
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    size: usize,
    pub head: Node<T>,
    // pub tail: Option<Box<Node<T>>>,
}

impl<T: std::marker::Copy> LinkedList<T> {
    pub fn new(n: T) -> LinkedList<T> {
        LinkedList {
            size: 0,
            head: Node::new(n),
            // tail: Some(start),
        }
    }
    pub fn len(&self) -> usize {self.size}
    pub fn push(&mut self, n: T) {
        self.head.push(n);
        self.size += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.size -= 1;
        self.head.pop()
    }
}

impl<T> Iterator for Node<T> {
    type Item = Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(box_node) => Some(*box_node),
        }
    }
}

fn main() {
    let lst = LinkedList::<i32>::new(10);
    println!("{:?}", lst);
}
use std::{cell::RefCell, rc::Rc, collections::linked_list};
// Rc（引用计数器指针） RefCell来保证内部可变性，即可以在内容改变数据 RefCell<T> 代表其数据的唯一的所有权
type Link<T> = Option<Rc<RefCell<Box<Node<T>>>>>;
struct Node<T> {
    data: Rc<T>,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: Rc::new(data),
            prev: None,
            next: None,
        }
    }

    fn set_prev(&mut self, other: &Link<T>) {
        self.prev = other.clone();
    }

    fn set_next(&mut self, other: &Link<T>) {
        self.next = other.clone();
    }
    fn get_prev(&self) -> Link<T> {
        self.prev.clone()
    }

    fn get_next(&self) -> Link<T> {
        self.next.clone()
    }

    pub fn new_link(data: T) -> Link<T> {
        Some(Rc::new(RefCell::new(Box::new(Node::new(data)))))
    }
}


// https://cloud.tencent.com/developer/article/2333391?areaId=106001
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push(&mut self,data:T){
        let new_node:Link<T> = Node::new_link(data);
        self.len += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
            return;
        }
        self.tail.as_ref().unwrap().borrow_mut().set_next(&new_node);
        new_node.as_ref().unwrap().borrow_mut().set_prev(&self.tail);

        self.tail = new_node;
    }
}



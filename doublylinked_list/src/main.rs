use core::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            prev: None,
            next: None,
            data: element,
        }
    }

    pub fn new_with_prev(element: T, prev: &Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self { 
            data: element,
            next: None,
            prev: Option::Some(Rc::downgrade(prev.as_ref().unwrap()))
        }
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Node<T>>>) {
        self.next = Option::Some(next);
    }
}

struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> DoublyLinkedList<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        return self.len.clone();
    }

    pub fn append(&mut self, element: T) {
        match self.len {
            0 => {
                let new_node = Node::new(element);
                self.first = Option::Some(Rc::new(RefCell::new(new_node)));
                self.len += 1;
            }

            1 => {
                let new_node = Node::new_with_prev(element, &self.first);
                self.last = Option::Some(Rc::new(RefCell::new(new_node)));
                self.first
                    .as_deref()
                    .unwrap()
                    .borrow_mut()
                    .set_next(self.last.as_ref().unwrap().clone());
                self.len += 1;
            }

            _ => {
                let new_node = Rc::new(
                    RefCell::new(
                        Node::new_with_prev(
                            element, &self.last
                        )
                    )
                );
                
                self.last
                    .as_deref()
                    .unwrap()
                    .borrow_mut()
                    .set_next(new_node.clone());

                self.last = Option::Some(new_node);
                self.len += 1;
            }
        }
    }

    fn get_at(&self, index: usize) -> T {
        
        if index > self.len || index < 0 {
            panic!("Index out of range");
        }

        let mut element = self.first.clone();
        let mut counter = 0;

        while counter != self.len && counter != index {
            element = element.unwrap().as_ref().borrow().next.clone();
            counter += 1;
        }

        return element.as_ref().unwrap().borrow().data.clone();
    }
}

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    list.append(32);
    list.append(23);
    list.append(7);
    list.append(6);
    list.append(3453);
    list.append(7657);
    println!("Elements of list are: ");
    for i in 0..list.len() {
        println!("Element [{i}]: {:?}", list.get_at(i));
    }
}

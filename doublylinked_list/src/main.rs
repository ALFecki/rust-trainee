use std::{rc::{Rc, Weak}, borrow::Borrow};
use core::cell::RefCell;


struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self { prev: None, next: None, data: element }
    }

    pub fn new_with_prev(element: T, prev: Weak<RefCell<Node<T>>>) -> Self {
        Self {next: None, prev: Option::Some(prev), data: element}
    }

    // pub fn get_data(&self) -> Rc<RefCell<T>> {
    //     return Rc::new(RefCell::new(self.data));
    // }

    pub fn set_next(&mut self, next: Rc<RefCell<Node<T>>>) {
        self.next = Option::Some(next);
    }

    // pub fn set_prev(&mut self, prev: Option<Weak<RefCell<Node<T>>>>) {
    //     self.prev = prev;
    // }
}

impl<T> Iterator for Node<T> where T: Copy {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        
        return self.next.clone();
    }
}

struct DoublyLinkedList<T> {
    first : Option<Rc<RefCell<Node<T>>>>,
    last : Option<Rc<RefCell<Node<T>>>>,
    len : usize
}



impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { first: None, last: None, len: 0 }
    }

    pub fn append(&mut self, element : T) {
        if self.len == 0 {
            let new_node = Node::new(element);
            self.first = Option::Some(Rc::new(RefCell::new(new_node)));
            self.len += 1;
        } else if self.len == 1 {
            let new_node = Node {
                data: element,
                prev: Option::Some(Rc::downgrade(&self.first.as_ref().unwrap())),
                next: None
            };
            self.last = Option::Some(Rc::new(RefCell::new(new_node)));
            self.first
                .as_deref()
                .unwrap()
                .borrow_mut()
                .set_next(self.last.as_ref().unwrap().clone());
            self.len += 1;
        } else {
            let new_node = Rc::new(RefCell::new(Node {
                data: element,
                prev: Option::Some(Rc::downgrade(&self.last.as_ref().unwrap())),
                next: None
            }));
            self.last
                .as_deref()
                .unwrap()
                .borrow_mut()
                .set_next(new_node.clone());
            self.last = Option::Some(new_node);
            self.len += 1;
        }
    }
    

    fn get_at(&self, index: usize) -> Rc<RefCell<Node<T>>> {
        let mut iterator = self.first.iter();
        let mut element = iterator.next();
        let mut counter = 0;
        while (counter != self.len - 1 && counter != index) {
            element = iterator.next();
            counter += 1;
        }
        return element.unwrap().clone();
    }
}

fn main() {
    let mut list :DoublyLinkedList<i32> = DoublyLinkedList::new();
    list.append(32);
    list.append(23);
    list.append(7);
    println!("Element {}", list.get_at(1).as_ref().borrow().data);

    // println!("{}",data);
    // list.append(String::from("23"));
    // let data = &list.first.as_ref().unwrap().as_ref().borrow().next.as_ref().unwrap().as_ref().borrow().data;
    // println!("{}",data);
    // list.append(String::from("6"));
    // println!("Elements: {}, {}, {}, {}", list.get_at(&0), list.get_at(&2), list.get_at(&3), list.get_at(&4))
    
}

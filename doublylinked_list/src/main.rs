use core::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::rc::{Rc, Weak};

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

struct NodeMutRef<T> {
    data: NonNull<Node<T>>
}

impl<T> NodeMutRef<T> {
    fn new(data: &mut Node<T>) -> Self {
        Self {
            data: NonNull::<Node<T>>::new(data as *mut Node<T>).expect("data is null")
        }
    }
}

impl<T> Deref for NodeMutRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { return self.data.as_ref().data.borrow(); }
    }
}

impl<T> DerefMut for NodeMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { return  self.data.as_mut().data.borrow_mut() }
    }
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
            prev: Option::Some(Rc::downgrade(prev.as_ref().unwrap())),
        }
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Node<T>>>) {
        self.next = Option::Some(next);
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return self.data.borrow();
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
                self.first = Some(Rc::new(RefCell::new(new_node)));
                self.len += 1;
            }

            1 => {
                let new_node = Node::new_with_prev(element, &self.first);
                self.last = Some(Rc::new(RefCell::new(new_node)));
                if let (Some(val_first), Some(val_last)) = (&self.first.clone(), &self.last.clone()) {
                    // val_first.borrow_mut().get_mut().set_next(val_last);

                    val_first.as_ref().borrow_mut().set_next(val_last.clone());
                }
                self.len += 1;
            }

            _ => {
                let new_node = Rc::new(RefCell::new(Node::new_with_prev(element, &self.last)));

                if let Some(val) = &self.last.clone() {
                    val.as_ref().borrow_mut().set_next(new_node.clone());
                    // val.as_ref().get_mut().set_next(new_node.clone());
                }

                self.last = Some(new_node);
                self.len += 1;
            }
        }
    }

    fn get_at(&self, index: usize) -> NodeMutRef<T> {
        if index > self.len || index < 0 {
            panic!("Index out of range");
        }

        let mut element = self.first.clone();
        let mut counter = 0;

        while counter != self.len && counter != index {
            if let Some(element_val) = element {
                element = element_val.as_ref().borrow().next.clone();
            }
            counter += 1;
        }

        match element.clone() {
            Some(element_val) => return NodeMutRef::new(element_val.as_ref().borrow_mut().deref_mut()),
            None => panic!("Data is invalid!"),
        };
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
        println!("Element [{i}]: {:?}", *list.get_at(i));
    }
}

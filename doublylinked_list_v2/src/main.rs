use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
struct Node<'a, T> {
    pub data: T,
    pub next: Option<&'a Node<'a, T>>,
    pub prev: Option<&'a Node<'a, T>>,
}


struct DoublyLinkedList<'a, T> {
    pub first: Option<Rc<RefCell<Node<'a, T>>>>,
    pub last: Option<Rc<RefCell<Node<'a, T>>>>,
    len: usize,
}


impl<T> DoublyLinkedList<T> {

    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            len: 0,
        }
    }


    pub fn append(&mut self, element: T) {
        match self.len {

            0 => {
                let new_node = Node::new(element);
                self.first = Option::Some(Rc::new(RefCell::new(new_node)));
                self.len += 1;
            }

            1 => {
                let new_node = Node::new_with_prev(element, self.first.as_ref().unwrap().borrow());
                self.last = Option::Some(Rc::new(RefCell::new(new_node)));
                self.first
                    .as_deref()
                    .unwrap()
                    .borrow_mut()
                    .set_next(self.last.as_ref().unwrap().as_ref().borrow().deref());
                self.len += 1;
            }

            _ => {
                let new_node = Rc::new(
                    RefCell::new(
                        Node::new_with_prev(
                            element, &self.last.as_ref().unwrap().borrow().deref()
                        )
                    )
                );

                self.last
                    .as_deref()
                    .unwrap()
                    .borrow_mut()
                    .set_next(new_node.borrow().deref().clone().borrow());

                self.last = Option::Some(new_node);
                self.len += 1;
            }
        }
    }

}



fn main() {
    let a = DoublyLinkedList{ first: Some(Rc::new(RefCell::new(Node{data: 2, next: None, prev: None}))), last: None, len: 1};
}

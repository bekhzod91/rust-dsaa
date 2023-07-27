use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl <T>Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None
        }
    }
}


pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}


impl <T: Clone>LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node::new(data)));

        match self.head.take() {
            Some(head) => {
                head.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&head));

                self.head = Some(Rc::clone(&node));
                self.size += 1;
            },
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
                self.size += 1;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                if let Some(prev_node) = &head.borrow().prev {
                    self.head = Some(Rc::clone(&prev_node));
                }

                self.size -= 1;

                if self.size == 0 {
                    self.tail = None;
                }
                return Some(head.borrow().value.to_owned());
            },
            None => {
                return None;
            }
        }
    }

    pub fn shift(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(Rc::clone(&tail));

                self.tail = Some(Rc::clone(&node));
                self.size += 1;
            }, 
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));

                self.size += 1;
            }
        }
    }


    pub fn unshift(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                if let Some(next_node) = &tail.borrow().next {
                    self.tail = Some(Rc::clone(&next_node));
                }

                self.size -= 1;

                if self.size == 0 {
                    self.head = None;
                }
                return Some(tail.borrow().value.to_owned());
            },
            None => {
                return None;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let mut linked_list = LinkedList::new();
        
        assert_eq!(true, linked_list.is_empty());

        linked_list.push(1);
        assert_eq!(false, linked_list.is_empty());

        linked_list.pop();
        assert_eq!(true, linked_list.is_empty());
    }

    #[test]
    fn test_len() {
        let mut linked_list = LinkedList::new();

        // Push
        linked_list.push(3);
        assert_eq!(1, linked_list.len());
        linked_list.push(2);
        assert_eq!(2, linked_list.len());

        // Shift
        linked_list.shift(3);
        assert_eq!(3, linked_list.len());
        linked_list.shift(4);
        assert_eq!(4, linked_list.len());

        // Pop
        linked_list.pop();
        assert_eq!(3, linked_list.len());
        linked_list.pop();
        assert_eq!(2, linked_list.len());

        // Unshift
        linked_list.unshift();
        assert_eq!(1, linked_list.len());

        linked_list.unshift();
        assert_eq!(0, linked_list.len());
    
    }

    #[test]
    fn test_push() {
        let mut linked_list = LinkedList::new();

        // Push to head
        linked_list.push(3);
        linked_list.push(2);
        linked_list.push(1);

        assert_eq!(Some(1), linked_list.pop());
        assert_eq!(Some(2), linked_list.pop());
        assert_eq!(Some(3), linked_list.pop());
    }

    #[test]
    fn test_pop() {
        let mut linked_list = LinkedList::new();

        // Push to tail
        linked_list.push(3);
        linked_list.push(2);
        linked_list.push(1);

        // Push to tail
        linked_list.shift(4);
        linked_list.shift(5);
        linked_list.shift(6);

        assert_eq!(Some(1), linked_list.pop());
        assert_eq!(Some(2), linked_list.pop());
        assert_eq!(Some(3), linked_list.pop());
        assert_eq!(Some(4), linked_list.pop());
        assert_eq!(Some(5), linked_list.pop());
        assert_eq!(Some(6), linked_list.pop());
    }


    #[test]
    fn test_shift() {
        let mut linked_list = LinkedList::new();

        // Push to head
        linked_list.shift(3);
        linked_list.shift(2);
        linked_list.shift(1);

        assert_eq!(Some(1), linked_list.unshift());
        assert_eq!(Some(2), linked_list.unshift());
        assert_eq!(Some(3), linked_list.unshift());
    }

    #[test]
    fn test_unshift() {
        let mut linked_list = LinkedList::new();

        // Push to head
        linked_list.push(3);
        linked_list.push(2);
        linked_list.push(1);

        // Push to tail
        linked_list.shift(4);
        linked_list.shift(5);
        linked_list.shift(6);

        assert_eq!(Some(6), linked_list.unshift());
        assert_eq!(Some(5), linked_list.unshift());
        assert_eq!(Some(4), linked_list.unshift());
        assert_eq!(Some(3), linked_list.unshift());
        assert_eq!(Some(2), linked_list.unshift());
        assert_eq!(Some(1), linked_list.unshift());
    }
}
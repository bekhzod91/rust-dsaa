// Purpose: Stack implementation using linked list
// Author: Bekhzod Tillakhanov


// Node struct for linked list
#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T
}


// Stack struct
#[derive(Debug)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // Pushes value to the stack
    //
    // # Arguments
    // * `value` - Value to be pushed to the stack
    //
    // Example:
    // ```
    // let mut stack = Stack::new();
    // stack.push(1);
    // ```
    pub fn push(&mut self, value: T) {
        let node = Node {
            next: self.head.take(),
            value
        };

        self.head = Some(Box::new(node));
    }

    // Returns the value of the top element
    //
    // Example:
    // ```
    // let mut stack = Stack::new();
    // stack.push(1);
    // assert_eq!(stack.peek(), Some(1));
    // ```
    pub fn peek(&mut self) -> Option<T> {
        match &self.head {
            None => None,
            Some(node) => Some(node.value.to_owned())
        }
    }

    // Removes the top element from the stack
    //
    // Example:
    // ```
    // let mut stack = Stack::new();
    // stack.push(1);
    // assert_eq!(stack.pop(), Some(1));
    // ```
    pub fn pop(&mut self) -> Option<T> {
        let new_head = self.head.take();

        match new_head {
            None => None,
            Some(mut node) => {
                self.head = node.next.take();

                Some(node.value.to_owned())
            }
        }
    }

    // Checks if the stack is empty
    //
    // Example:
    // ```
    // let mut stack = Stack::new();
    // assert_eq!(stack.is_empty(), true);
    // ```
    pub fn is_empty(&mut self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_to_stack() {
        let mut stack = Stack::new();

        stack.push(1);
        assert_eq!(stack.peek(), Some(1));

        stack.push(2);
        assert_eq!(stack.peek(), Some(2));

        stack.push(3);
        assert_eq!(stack.peek(), Some(3));
    }

    #[test]
    fn test_pop_from_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_is_empty_on_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.is_empty(), true);
        
        stack.push(1);
        assert_eq!(stack.is_empty(), false);

        let _ = stack.pop();
        assert_eq!(stack.is_empty(), true);
    }
}

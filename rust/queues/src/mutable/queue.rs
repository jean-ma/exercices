use std::{cell::RefCell, rc::Rc};

pub struct Queue<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn enqueue(&mut self, t: T) {
        let new = Rc::new(Node {
            value: t,
            next: RefCell::new(None),
        });
        self.size += 1;

        match &self.tail {
            Some(tail) => {
                *tail.next.borrow_mut() = Some(Rc::clone(&new));
                self.tail = Some(Rc::clone(&new));
            }
            None => {
                let t = Rc::new(new);
                self.tail = Some(Rc::clone(&t));
                self.head = Some(Rc::clone(&t));
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<Rc<Node<T>>> {
        if let Some(h) = self.head.take() {
            let ans = Rc::clone(&h);

            if let Some(a) = ans.next.take() {
                self.head = Some(Rc::clone(&a));
            } else {
                self.head = None;
                self.tail = None;
            };

            self.size -= 1;

            return Some(ans);
        }
        return None;
    }
}

pub struct Node<T> {
    pub value: T,
    next: RefCell<Option<Rc<Node<T>>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue() -> Result<(), String> {
        let mut q = Queue::<i32>::new();
        assert_eq!(q.size(), 0);

        match q.dequeue() {
            Some(_) => Err(String::from("Queue should be empty")),
            None => Ok(()),
        }
    }

    #[test]
    fn enqueue_dequeue() {
        let mut q = Queue::<i32>::new();
        q.enqueue(32);

        assert_eq!(q.size(), 1);

        let dequeue = match q.dequeue() {
            Some(node) => {
                if node.value == 32 {
                    Ok(())
                } else {
                    Err(String::from(
                        "Queue should have one element with value == 32",
                    ))
                }
            }
            None => Err(String::from("Queue should have one element")),
        };
        assert!(dequeue.is_ok(), "{}", dequeue.unwrap_err());

        assert_eq!(q.size(), 0);
    }

    #[test]
    fn enqueue_double_dequeue() {
        let mut q = Queue::<i32>::new();
        q.enqueue(32);

        assert_eq!(q.size(), 1);

        let dequeue = match q.dequeue() {
            Some(node) => {
                if node.value == 32 {
                    Ok(())
                } else {
                    Err(String::from(
                        "Queue should have one element with value == 32",
                    ))
                }
            }
            None => Err(String::from("Queue should have one element")),
        };
        assert!(dequeue.is_ok(), "{}", dequeue.unwrap_err());

        assert_eq!(q.size(), 0);

        let dequeue = match q.dequeue() {
            Some(_) => Err(String::from("Queue should have ZERO element")),
            None => Ok(()),
        };
        assert!(dequeue.is_ok(), "{}", dequeue.unwrap_err());

        assert_eq!(q.size(), 0);

        q.enqueue(32);
        assert_eq!(q.size(), 1);
    }
}

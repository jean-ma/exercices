use crate::immutable::queue::Node::{Cons, Nil};

enum Node<T> {
    Cons(T, Box<Node<T>>),
    Nil,
}

pub struct Queue<T> {
    inn: Node<T>,
    out: Node<T>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            inn: Nil,
            out: Nil,
            size: 0,
        }
    }

    pub fn dequeue(self) -> (Option<T>, Queue<T>) {
        if let Cons(v, next) = self.out {
            return (
                Some(v),
                Queue {
                    out: *next,
                    size: self.size - 1,
                    ..self
                },
            )
        }

        if let Cons(_, _) = self.inn {
            return self.swap().dequeue()
        }

        (None, self)
    }

    fn swap(self) -> Queue<T> {
        let mut new_out = Nil;
        let mut read_inn = self.inn;

        loop {
            if let Cons(t, next) = read_inn {
                new_out = Cons(t, Box::new(new_out));
                read_inn = *next;
            } else {
                break;
            }
        }

        Queue { inn: Nil, out: new_out, size: self.size }
    }

    pub fn enqueue(self, t: T) -> Queue<T> {
        Queue {
            inn: Cons(t, Box::new(self.inn)),
            size: self.size + 1,
            ..self
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue() -> Result<(), String> {
        let q = Queue::<i32>::new();
        assert_eq!(q.size(), 0);

        match q.dequeue() {
            (Some(_), _) => Err(String::from("Queue should be empty")),
            (None, _) => Ok(()),
        }
    }

    #[test]
    fn enqueue_dequeue() {
        let q = Queue::<i32>::new();
        let q = q.enqueue(32);

        assert_eq!(q.size(), 1);

        let (v, q) = q.dequeue();

        let dequeue = match v {
            Some(32) => Ok(()),
            Some(_) => Err(String::from("Queue should have one element")),
            None => Err(String::from("Queue should have one element"))
        };
        assert!(dequeue.is_ok(), "{}", dequeue.unwrap_err());

        assert_eq!(q.size(), 0);
    }
}

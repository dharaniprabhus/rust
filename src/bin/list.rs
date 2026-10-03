struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value: value,
            next: self.head.take(),
        }));
    }

    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    fn pop(&mut self) {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
        }
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(10);
    s.push(12);
    s.push(13);
    s.push(14);
    s.push(15);
    while let Some(&val) = s.peek() {
        s.pop();
        print!("{} ", val);
    }
}

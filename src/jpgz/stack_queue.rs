struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { vec: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        Stack {
            vec: Vec::with_capacity(capacity),
        }
    }
    // fn push ...
    fn pop(&mut self) -> Option<T> {
        // self.vec.drain(range)
        self.vec.pop()
    }
}

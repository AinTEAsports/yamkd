pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self { Stack(Vec::new()) }

    pub fn push(&mut self, t: T) { self.0.push(t) }
    pub fn pop(&mut self) -> Option<T> { self.0.pop() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn peek(&self) -> Option<&T> { self.0.last() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

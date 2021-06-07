use thiserror::Error;

use super::Val;

#[derive(Default, Debug)]
pub struct Stack {
    data: Vec<Val>,
}

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Stack is empty.")]
    EmptyStackError,
}

impl Stack {
    pub fn push(&mut self, value: Val) {
        self.data.push(value);
    }
    pub fn pop(&mut self) -> Result<Val, StackError> {
        self.data.pop().ok_or(StackError::EmptyStackError)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn take(self) -> Vec<Val> {
        self.data
    }

    pub fn truncate(&mut self, length: usize) {
        self.data.truncate(length);
    }
}

impl Extend<Val> for Stack {
    fn extend<T: IntoIterator<Item = Val>>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Val>,
    {
        self.data.extend(iter);
    }
}
impl<'a> Extend<&'a Val> for Stack {
    fn extend<T: IntoIterator<Item = &'a Val>>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a Val>,
    {
        self.data.extend(iter);
    }
}

impl From<Stack> for Vec<Val> {
    fn from(from: Stack) -> Vec<Val> {
        from.data
    }
}

use std::cell::Cell;

use super::{Execute, Flow, Frame};
use crate::stack::Stack;

#[derive(Clone)]
pub struct VecFlow<'a, T: Execute> {
    instrs: &'a Vec<T>,
    index: usize,
}

impl<'a, T: Execute> Flow<'a> for VecFlow<'a, T> {
    fn next(&'a self) -> Option<&'a dyn Flow<'a>> {}

    fn update_next(&'a self, next: Option<&'a dyn Flow<'a>>) {}
}

impl<T: Execute> Execute for VecFlow<'_, T> {
    fn execute(&self, stack: &mut Stack, frame: &mut Frame) {}
}

use std::cell::Cell;

use super::{Execute, Flow, Frame};
use crate::stack::Stack;

#[derive(Clone)]
pub struct InstrFlow<'a> {
    instr: &'a dyn Execute,
    next: Cell<Option<&'a dyn Flow<'a>>>,
}

impl<'a> Flow<'a> for InstrFlow<'a> {
    fn next(&'a self) -> Option<&'a dyn Flow<'a>> {
        self.next.get()
    }

    fn update_next(&'a self, next: Option<&'a dyn Flow<'a>>) {
        self.next.set(next);
    }
}

impl Execute for InstrFlow<'_> {
    fn execute(&self, stack: &mut Stack, frame: &mut Frame) {
        self.instr.execute(stack, frame);
        if let Some(instr) = self.next.get() {
            instr.execute(stack, frame);
        }
    }
}

impl<'a> InstrFlow<'a> {
    pub fn new(instr: &'a dyn Execute) -> InstrFlow {
        InstrFlow {
            instr,
            next: Cell::new(None),
        }
    }

    pub fn update_and_link(&'a self, link: &'a dyn Flow<'a>) {
        link.update_next(self.next.get());
        self.next.set(Some(link));
    }
}

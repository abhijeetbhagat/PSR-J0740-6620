use cursive::view::View;
use cursive::Printer;

pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub(crate) struct Calc {
    pub(crate) op1: u64,
    pub(crate) op2: u64,
    pub(crate) should_clear: bool,
    pub(crate) op: Op,
    pub(crate) bin: Vec<bool>,
}

struct Numpad;

impl View for Numpad {
    fn draw(&self, printer: &Printer<'_, '_>) {}
}

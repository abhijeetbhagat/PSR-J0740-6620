use cursive::view::View;
use cursive::views::{RadioButton, RadioGroup};
use cursive::Printer;
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Lsh,
    Rsh,
    Not,
    And,
    Or,
}

#[derive(PartialEq)]
pub(crate) enum Mode {
    Hex,
    Dec,
}

pub(crate) struct Calc {
    pub(crate) op1: u64,
    pub(crate) op2: u64,
    pub(crate) should_clear: bool,
    pub(crate) op: Op,
    pub(crate) bin: Vec<bool>,
    pub(crate) mode: Mode,
    pub(crate) mode_group: RadioGroup<Mode>,
}

struct Numpad;

impl View for Numpad {
    fn draw(&self, printer: &Printer<'_, '_>) {}
}

use anyhow::Result;
use cursive::event::Key;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette, PaletteColor};
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, OnEventView, Panel, RadioButton, RadioGroup, TextView,
    ViewRef,
};
use std::rc::Rc;
mod numpad;
use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::vec::Vec2;
use cursive::Cursive;
use cursive::Printer;
use numpad::{Calc, Mode, Op};

fn main() -> Result<()> {
    let mut siv = cursive::default();
    let mut layout = LinearLayout::vertical();

    let bin_board_row_1 = create_bin_board_row(0, 32);
    let bin_board_row_2 = create_bin_board_row(32, 64);

    let first = LinearLayout::horizontal()
        .child(Button::new_raw(" 1 ", |s| display_char(s, '1')))
        .child(Button::new_raw(" 2 ", |s| display_char(s, '2')))
        .child(Button::new_raw(" 3 ", |s| display_char(s, '3')))
        .child(Button::new_raw(" C ", |s| display_char(s, 'C')).with_name("C"))
        .child(Button::new_raw(" + ", |s| store_op(s, Op::Add)))
        .child(Button::new_raw(" << ", |s| store_op(s, Op::Lsh)))
        .child(Button::new_raw(" >> ", |s| store_op(s, Op::Rsh)));

    let second = LinearLayout::horizontal()
        .child(Button::new_raw(" 4 ", |s| display_char(s, '4')))
        .child(Button::new_raw(" 5 ", |s| display_char(s, '5')))
        .child(Button::new_raw(" 6 ", |s| display_char(s, '6')))
        .child(Button::new_raw(" D ", |s| display_char(s, 'D')).with_name("D"))
        .child(Button::new_raw(" - ", |s| store_op(s, Op::Sub)))
        .child(Button::new_raw(" & ", |s| store_op(s, Op::And)))
        .child(Button::new_raw(" | ", |s| store_op(s, Op::Or)));

    let third = LinearLayout::horizontal()
        .child(Button::new_raw(" 7 ", |s| display_char(s, '7')))
        .child(Button::new_raw(" 8 ", |s| display_char(s, '8')))
        .child(Button::new_raw(" 9 ", |s| display_char(s, '9')))
        .child(Button::new_raw(" E ", |s| display_char(s, 'E')).with_name("E"))
        .child(Button::new_raw(" * ", |s| store_op(s, Op::Mul)));

    let fourth = LinearLayout::horizontal()
        .child(Button::new_raw(" 0 ", |s| display_char(s, '0')))
        .child(Button::new_raw(" A ", |s| display_char(s, 'A')).with_name("A"))
        .child(Button::new_raw(" B ", |s| display_char(s, 'B')).with_name("B"))
        .child(Button::new_raw(" F ", |s| display_char(s, 'F')).with_name("F"))
        .child(Button::new_raw(" / ", |s| store_op(s, Op::Div)))
        .child(Button::new_raw(" = ", |s| perform_calc(s)));

    let input_row = LinearLayout::horizontal()
        .child(
            EditView::new()
                .style(ColorStyle::new(
                    Color::Dark(BaseColor::Black),
                    Color::Rgb(26, 128, 111),
                ))
                .with_name("input")
                .fixed_width(20),
        )
        .child(Button::new_raw(" AC ", |s| all_clear(s)))
        .child(Button::new_raw(" cp ", |s| cp(s)));

    let mut mode_group: RadioGroup<Mode> = RadioGroup::new().on_change(on_mode_change);
    let mode_row = LinearLayout::horizontal()
        .child(mode_group.button(Mode::Hex, "Hex "))
        .child(mode_group.button(Mode::Dec, "Dec "));

    layout = layout.child(input_row);
    layout = layout.child(bin_board_row_1);
    layout = layout.child(TextView::new("63      56      48     40     32").style(
        ColorStyle::new(Color::Rgb(50, 99, 81), Color::Rgb(77, 255, 195)),
    ));
    layout = layout.child(bin_board_row_2);
    layout = layout.child(TextView::new("31      24      16     8       0").style(
        ColorStyle::new(Color::Rgb(50, 99, 81), Color::Rgb(77, 255, 195)),
    ));

    layout = layout.child(Panel::new(mode_row));
    layout = layout.child(first);
    layout = layout.child(second);
    layout = layout.child(third);
    layout = layout.child(fourth);
    let button = Button::new_raw(" Quit ", |s| s.quit());
    layout = layout.child(button);

    let state = Calc {
        op1: 0,
        op2: 0,
        op: Op::Add,
        should_clear: false,
        bin: vec![false; 64],
        mode_group,
        mode: Mode::Dec,
        shortcut: String::new(),
        shortcut_activated: false,
    };
    siv.set_user_data(state);

    siv.add_layer(
        OnEventView::new(layout)
            .on_event(':', |s| {
                s.with_user_data(|data: &mut Calc| {
                    data.shortcut_activated = true;
                });
            })
            .on_event('0', |s| append_shortcut(s, '0'))
            .on_event('1', |s| append_shortcut(s, '1'))
            .on_event('2', |s| append_shortcut(s, '2'))
            .on_event('3', |s| append_shortcut(s, '3'))
            .on_event('4', |s| append_shortcut(s, '4'))
            .on_event('5', |s| append_shortcut(s, '5'))
            .on_event('6', |s| append_shortcut(s, '6'))
            .on_event('7', |s| append_shortcut(s, '7'))
            .on_event('8', |s| append_shortcut(s, '8'))
            .on_event('9', |s| append_shortcut(s, '9'))
            .on_event(' ', |s| {
                let state = s.user_data::<Calc>().unwrap().clone();
                if state.shortcut_activated && !state.shortcut.is_empty() {
                    let bit_num: usize = state.shortcut.parse().unwrap();
                    toggle_bit(s, 63 - bit_num);
                }
            }),
    );
    siv.add_global_callback('q', |s| s.quit());
    siv.focus_name("0").unwrap();

    let mut theme = siv.current_theme().clone();
    theme.borders = BorderStyle::Simple;
    let mut palette = Palette::default();
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Rgb(77, 255, 195);
    theme.palette = palette;
    siv.set_theme(theme);

    siv.run();
    Ok(())
}

fn append_shortcut(s: &mut Cursive, c: char) {
    s.with_user_data(|data: &mut Calc| {
        if data.shortcut_activated && !data.shortcut.len() > 2 {
            data.shortcut.push(c)
        }
    });
}

fn calculate(data: &Calc) -> u64 {
    match data.op {
        Op::Add => data.op1 + data.op2,
        Op::Sub => data.op1 - data.op2,
        Op::Mul => data.op1 * data.op2,
        Op::Div => data.op1 / data.op2,
        Op::Lsh => data.op1 << data.op2,
        Op::Rsh => data.op1 >> data.op2,
        Op::And => data.op1 & data.op2,
        Op::Or => data.op1 | data.op2,
        Op::Not => !data.op1,
    }
}

#[inline]
fn toggle_bit(s: &mut Cursive, i: usize) {
    if i > 63 {
        return;
    }

    let mut button = s.find_name::<Button>(&i.to_string()).unwrap();

    /*
    let (d, v) = {
        let mut data = s.user_data::<Calc>().unwrap();
        data.bin[i] = data.bin[i] & 1;
        let d = Dialog::text(&format!("val is {}", data.bin[i])).dismiss_button("OK");
        (d, data.bin[i])
    };
    s.add_layer(d);
    */

    let data = s.user_data::<Calc>().unwrap();
    button.set_label_raw(if data.bin[i] { "0" } else { "1" });
    data.bin[i] = !data.bin[i];
    let mut result = 0u64;
    for idx in (0..64).rev() {
        result |= (data.bin[idx] as u64) << 63 - idx;
    }
    data.shortcut.clear();
    data.shortcut_activated = false;

    let mut tb: ViewRef<EditView> = s.find_name::<EditView>("input").unwrap();
    tb.set_content(&result.to_string());
}

fn display_char(s: &mut Cursive, c: char) {
    let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
    let input = &*tb.get_content();
    let mut input = input.clone();
    let state = s.user_data::<Calc>().unwrap();
    if state.should_clear {
        input = "".into();
        state.should_clear = false;
    }
    input.push(c);

    tb.set_content(input.as_str());
}

fn store_op(s: &mut Cursive, op: Op) {
    let tb: ViewRef<EditView> = s.find_name("input").unwrap();
    let input = &*tb.get_content();
    if !input.is_empty() {
        s.with_user_data(|data: &mut Calc| {
            data.op1 = if *data.mode_group.selection() == Mode::Hex {
                u64::from_str_radix(&input, 16).unwrap()
            } else {
                input.parse().unwrap()
            };
            data.op = op;
            data.should_clear = true;
        });
    }
}

fn create_bin_board_row(lsb: usize, msb: usize) -> LinearLayout {
    let mut bin_board = LinearLayout::horizontal();
    for i in lsb..msb {
        bin_board = bin_board.child(
            Button::new_raw("0", move |s| {
                toggle_bit(s, i);
            })
            .with_name(&i.to_string()),
        );
    }
    bin_board
}

fn perform_calc(s: &mut Cursive) {
    let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();

    let input = &*tb.get_content();
    let input = input.clone();
    s.with_user_data(|data: &mut Calc| {
        data.op2 = if *data.mode_group.selection() == Mode::Hex {
            u64::from_str_radix(&input, 16).unwrap()
        } else {
            input.parse().unwrap()
        };
        data.op1 = calculate(data);
        let formatted_output = if *data.mode_group.selection() == Mode::Hex {
            format!("{:X}", data.op1)
        } else {
            data.op1.to_string()
        };
        tb.set_content(&formatted_output);
        data.should_clear = true;
    });
}

fn all_clear(s: &mut Cursive) {
    let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
    s.with_user_data(|data: &mut Calc| {
        data.op2 = 0;
        data.op1 = 0;
        tb.set_content("");
    });
}

fn cp(s: &mut Cursive) {
    let tb: ViewRef<EditView> = s.find_name("input").unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let input = &*tb.get_content();
    let input = input.clone();
    ctx.set_contents(input).unwrap();
}

fn on_mode_change(s: &mut Cursive, mode: &Mode) {
    let buttons = ["A", "B", "C", "D", "E", "F"];
    let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
    let input = &*tb.get_content();
    let input = input.clone();

    let input = if *mode == Mode::Dec {
        for button in buttons.iter() {
            let mut button: ViewRef<Button> = s.find_name(button).unwrap();
            button.disable();
        }
        u64::from_str_radix(&input, 16).map_or_else(|_| "".into(), |num| num.to_string())
    } else {
        for button in buttons.iter() {
            let mut button: ViewRef<Button> = s.find_name(button).unwrap();
            button.enable();
        }
        input
            .parse::<u64>()
            .map_or_else(|_| "".to_string(), |num| format!("{:X}", num))
    };
    tb.set_content(&input);
}

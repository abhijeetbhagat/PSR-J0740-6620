use anyhow::Result;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette, PaletteColor};
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView, ViewRef};
use std::rc::Rc;
mod numpad;
use cursive::vec::Vec2;
use cursive::Cursive;
use cursive::Printer;
use numpad::{Calc, Op};

fn main() -> Result<()> {
    let mut siv = cursive::default();
    let mut layout = LinearLayout::vertical();

    let state = Calc {
        op1: 0,
        op2: 0,
        op: Op::Add,
        should_clear: false,
        bin: vec![false; 64],
    };
    siv.set_user_data(state);

    let mut bin_board_row_1 = LinearLayout::horizontal();
    for i in 0..32 {
        bin_board_row_1 = bin_board_row_1.child(
            Button::new_raw("0", move |s| {
                bin_board_helper(s, i);
            })
            .with_name(&i.to_string()),
        );
    }

    let mut bin_board_row_2 = LinearLayout::horizontal();
    for i in 32..64 {
        bin_board_row_2 = bin_board_row_2.child(
            Button::new_raw("0", move |s| {
                bin_board_helper(s, i);
            })
            .with_name(&i.to_string()),
        );
    }

    let first = LinearLayout::horizontal()
        .child(Button::new_raw(" 1 ", |s| display_helper(s, '1')))
        .child(Button::new_raw(" 2 ", |s| display_helper(s, '2')))
        .child(Button::new_raw(" 3 ", |s| display_helper(s, '3')))
        .child(Button::new_raw(" C ", |s| display_helper(s, 'C')))
        .child(Button::new_raw(" + ", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = u64::from_str_radix(input, 16).unwrap();
                data.op = Op::Add;
                data.should_clear = true;
            });
        }));
    let second = LinearLayout::horizontal()
        .child(Button::new_raw(" 4 ", |s| display_helper(s, '4')))
        .child(Button::new_raw(" 5 ", |s| display_helper(s, '5')))
        .child(Button::new_raw(" 6 ", |s| display_helper(s, '6')))
        .child(Button::new_raw(" D ", |s| display_helper(s, 'D')))
        .child(Button::new_raw(" - ", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Sub;
                data.should_clear = true;
            });
        }));

    let third = LinearLayout::horizontal()
        .child(Button::new_raw(" 7 ", |s| display_helper(s, '7')))
        .child(Button::new_raw(" 8 ", |s| display_helper(s, '8')))
        .child(Button::new_raw(" 9 ", |s| display_helper(s, '9')))
        .child(Button::new_raw(" E ", |s| display_helper(s, 'E')))
        .child(Button::new_raw(" * ", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Mul;
                data.should_clear = true;
            });
        }));

    let fourth = LinearLayout::horizontal()
        .child(Button::new_raw(" 0 ", |s| display_helper(s, '0')))
        .child(Button::new_raw(" A ", |s| display_helper(s, 'A')))
        .child(Button::new_raw(" B ", |s| display_helper(s, 'B')))
        .child(Button::new_raw(" F ", |s| display_helper(s, 'F')))
        .child(Button::new_raw(" / ", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Div;
                data.should_clear = true;
            });
        }))
        .child(Button::new_raw(" = ", |s| {
            /*
            let mut theme = s.current_theme().clone();
            theme.borders = BorderStyle::Simple;
            let mut palette = Palette::default();
            palette[PaletteColor::Primary] = Color::Light(BaseColor::Red);
            palette[PaletteColor::View] = Color::Light(BaseColor::Red);
            theme.palette = palette;

            s.set_theme(theme);
            */
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let input = input.clone();
            s.with_user_data(|data: &mut Calc| {
                data.op2 = u64::from_str_radix(&input, 16).unwrap();
                data.op1 = calculate(data); // data.op1 + data.op2;
                tb.set_content(&data.op1.to_string());
            });
        }));

    layout = layout.child(EditView::new().with_name("input").fixed_width(20));
    layout = layout.child(bin_board_row_1);
    layout = layout.child(TextView::new("63      56      48     40     32"));
    layout = layout.child(bin_board_row_2);
    layout = layout.child(TextView::new("31      24      16     8       0"));
    layout = layout.child(first);
    layout = layout.child(second);
    layout = layout.child(third);
    layout = layout.child(fourth);
    let button = Button::new_raw(" Quit ", |s| s.quit());
    layout = layout.child(button);

    siv.add_layer(layout);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
    Ok(())
}

struct TestView {}

impl View for TestView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        printer.with_color(
            ColorStyle::new(Color::Dark(BaseColor::Red), Color::Dark(BaseColor::Black)),
            |printer| printer.print((0, 0), &format!("{:^4}", "abhi")),
        );
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (23, 8).into()
    }
}

fn calculate(data: &Calc) -> u64 {
    match data.op {
        Op::Add => data.op1 + data.op2,
        Op::Sub => data.op1 - data.op2,
        Op::Mul => data.op1 * data.op2,
        Op::Div => data.op1 / data.op2,
    }
}

#[inline]
fn bin_board_helper(s: &mut Cursive, i: usize) {
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

    let mut tb: ViewRef<EditView> = s.find_name::<EditView>("input").unwrap();

    let data = s.user_data::<Calc>().unwrap();
    button.set_label_raw(if data.bin[i] { "0" } else { "1" });
    data.bin[i] = !data.bin[i];
    let mut result = 0u64;
    for idx in (0..64).rev() {
        result |= (data.bin[idx] as u64) << 63 - idx;
    }
    tb.set_content(&result.to_string());
}

fn display_helper(s: &mut Cursive, c: char) {
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

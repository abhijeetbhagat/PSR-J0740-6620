use anyhow::Result;
use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor};
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView, ViewRef};
use std::rc::Rc;
mod numpad;
use numpad::Calc;

fn main() -> Result<()> {
    let mut siv = cursive::default();
    let mut layout = LinearLayout::vertical();

    let state = Calc {
        op1: 0,
        op2: 0,
        op: Op::Add,
        should_clear: false,
    };
    siv.set_user_data(state);

    let first = LinearLayout::horizontal()
        .child(Button::new_raw("1", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('1');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("2", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('2');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("3", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('3');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("+", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Add;
                data.should_clear = true;
            });
        }));
    let second = LinearLayout::horizontal()
        .child(Button::new_raw("4", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('4');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("5", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('5');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("6", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('6');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("-", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Sub;
                data.should_clear = true;
            });
        }));

    let third = LinearLayout::horizontal()
        .child(Button::new_raw("7", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('7');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("8", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('8');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("9", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('9');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("*", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Mul;
                data.should_clear = true;
            });
        }));

    let fourth = LinearLayout::horizontal()
        .child(Button::new_raw("0", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            let state = s.user_data::<Calc>().unwrap();
            if state.should_clear {
                input = "".into();
            }
            input.push('0');

            tb.set_content(input.as_str());
        }))
        .child(Button::new_raw("/", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.op = Op::Div;
                data.should_clear = true;
            });
        }))
        .child(Button::new_raw("=", |s| {
            let mut theme = s.current_theme().clone();
            theme.borders = BorderStyle::Simple;
            let mut palette = Palette::default();
            palette[PaletteColor::Primary] = Color::Light(BaseColor::Red);
            palette[PaletteColor::View] = Color::Light(BaseColor::Red);
            theme.palette = palette;

            s.set_theme(theme);
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let input = input.clone();
            s.with_user_data(|data: &mut Calc| {
                data.op2 = input.parse().unwrap();
                data.op1 = calculate(data); // data.op1 + data.op2;
                tb.set_content(&data.op1.to_string());
            });
        }));

    layout = layout.child(EditView::new().with_name("input").fixed_width(20));
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

fn calculate(data: &Calc) -> u64 {
    match data.op {
        Op::Add => data.op1 + data.op2,
        Op::Sub => data.op1 - data.op2,
        Op::Mul => data.op1 * data.op2,
        Op::Div => data.op1 / data.op2,
    }
}

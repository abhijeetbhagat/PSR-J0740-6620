use anyhow::Result;
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView, ViewRef};
use std::rc::Rc;

struct Calc {
    op1: u64,
    op2: u64,
    should_clear: bool,
}

fn main() -> Result<()> {
    let mut siv = cursive::default();
    let mut layout = LinearLayout::vertical();

    let state = Calc {
        op1: 0,
        op2: 0,
        should_clear: false,
    };
    siv.set_user_data(state);

    let first = LinearLayout::horizontal()
        .child(Button::new("1", |s| {
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
        .child(Button::new("2", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('2');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("3", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('3');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("+", |s| {
            let tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            s.with_user_data(|data: &mut Calc| {
                data.op1 = input.parse().unwrap();
                data.should_clear = true;
            });
        }));
    let second = LinearLayout::horizontal()
        .child(Button::new("4", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('4');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("5", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('5');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("6", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('6');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("-", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('3');

            tb.set_content(input.as_str());
        }));

    let third = LinearLayout::horizontal()
        .child(Button::new("7", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('7');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("8", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('8');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("9", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('9');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("*", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('3');

            tb.set_content(input.as_str());
        }));

    let fourth = LinearLayout::horizontal()
        .child(Button::new("0", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('0');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("/", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let mut input = input.clone();
            input.push('3');

            tb.set_content(input.as_str());
        }))
        .child(Button::new("=", |s| {
            let mut tb: ViewRef<EditView> = s.find_name("input").unwrap();
            let input = &*tb.get_content();
            let input = input.clone();
            s.with_user_data(|data: &mut Calc| {
                data.op2 = input.parse().unwrap();
                data.op1 = data.op1 + data.op2;
                tb.set_content(&data.op1.to_string());
            });
        }));

    layout = layout.child(EditView::new().with_name("input").fixed_width(20));
    layout = layout.child(first);
    layout = layout.child(second);
    layout = layout.child(third);
    layout = layout.child(fourth);

    siv.add_layer(layout);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
    Ok(())
}

use anyhow::Result;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
};
use tui::symbols::Marker;
use tui::widgets::canvas::{Canvas, Rectangle};

use std::io::{stdout, Write};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

struct App {
    input_mode: InputMode,
    input: String,
    messages: Vec<String>,
}

impl App {}

fn main() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut app = App {
        input: String::new(),
        input_mode: InputMode::Normal,
        messages: vec![],
    };

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let (msg, style) = match app.input_mode {
                InputMode::Normal => (
                    vec![
                        Span::raw("Press"),
                        Span::styled(" q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing"),
                    ],
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                InputMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to record the message"),
                    ],
                    Style::default(),
                ),
            };

            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);

            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let input = Paragraph::new(app.input.as_ref())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);

            match app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => {
                    f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1)
                }
            }

            let numpad = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Numpad"))
                .paint(|ctx| {
                    ctx.draw(&Rectangle {
                        x: 10.0,
                        y: 10.0,
                        width: 10.0,
                        height: 10.0,
                        color: Color::Red,
                    });
                    ctx.draw(&Rectangle {
                        x: 25.0,
                        y: 10.0,
                        width: 10.0,
                        height: 10.0,
                        color: Color::Red,
                    });
                })
                .x_bounds([10.0, 110.0])
                .y_bounds([10.0, 110.0]);

            f.render_widget(numpad, chunks[2]);
        })?;

        match read()? {
            Event::Key(input) => match app.input_mode {
                InputMode::Normal => match input.code {
                    KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input.code {
                    KeyCode::Enter => app.messages.push(app.input.drain(..).collect()),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    _ => {}
                },
            },
            Event::Mouse(input) => match input.kind {
                MouseEventKind::Down(_) => println!("{}, {}", input.column, input.row),
                _ => {}
            },
            _ => {}
        }
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}

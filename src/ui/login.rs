use crate::config::Config;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    io::{self, stdin},
    sync::Condvar,
};
use termion::{event::Key, input::TermRead};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

struct LoginPage {
    user_id: String,
}
impl LoginPage {
    fn new(user_id: String) -> Self {
        LoginPage { user_id }
    }
    pub fn paste_from_clipboard(&mut self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(payload) => {
                for c in payload.chars() {
                    self.user_id.push(c);
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        };
    }
}

pub fn run<B: Backend>(config: &mut Config, terminal: &mut Terminal<B>) {
    let stdin = stdin();
    let mut login_page = LoginPage::new(config.user_id.clone());
    let mut keys = stdin.keys();

    loop {
        draw(&login_page.user_id, terminal).unwrap();
        let key = keys.next().unwrap();
        match key {
            Ok(Key::Esc) => {
                terminal.clear().unwrap();
                break;
            }
            Ok(Key::Backspace) => {
                login_page.user_id.pop();
            }
            Ok(Key::Ctrl('v')) => login_page.paste_from_clipboard(),
            Ok(Key::Char('\n')) => {
                match confy::store(
                    "fbmark",
                    Config {
                        api_key: "".to_string(),
                        user_id: login_page.user_id.clone(),
                    },
                ) {
                    Ok(_) => {
                        terminal.clear().unwrap();
                        break;
                    }
                    Err(e) => {
                        tracing::error!("{}", e);
                    }
                }
            }
            Ok(Key::Char(c)) => {
                login_page.user_id.push(c);
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }
}

pub fn draw<B: Backend>(user_id: &str, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let input_panel = draw_input_panel(user_id);
            f.render_widget(input_panel, chunks[0]);
        })
        .unwrap();
    Ok(())
}

fn draw_input_panel(user_id: &str) -> Paragraph {
    let lines = Text::from(user_id);
    Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(Span::styled(
                "Input desired user_id from Tagpacker",
                Style::default().fg(Color::White),
            )),
    )
}

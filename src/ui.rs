use std::io;
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{
        Layout, 
        Constraint, 
        Direction
    },
    text::{Span, Spans, Text},
    style::{Style, Color},
};

use termion::raw::IntoRawMode;

pub fn draw() -> Result<(), io::Error> {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                ].as_ref()
            )
            .split(f.size());

        let input_string: String = app.input.iter().collect();
        let lines = Text::from((&input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Search",
                Style::default().fg(Color::White).bg(Color::Black),     
                ))
            );
        f.render_widget(input, chunks[0]);

        let block = Block::default()
             .title("Links")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
}
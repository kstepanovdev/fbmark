use std::io;
use tui::{
    Terminal,
    backend::Backend,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{
        Layout, 
        Constraint, 
        Direction
    },
    text::{Span, Spans, Text},
    style::{Style, Color},
};

use super::app::{App, Bookmark, Bookmarks};
use sublime_fuzzy::best_match;

pub fn draw<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
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

        let input_string = &app.search_string;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Search",
                Style::default().fg(Color::White).bg(Color::Black),     
                ))
            );
        f.render_widget(input, chunks[0]);

                
        let lines = get_lines(&app.bookmarks, &app.search_string);
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Search",
                Style::default().fg(Color::White).bg(Color::Black),     
                ))
            );
        f.render_widget(input, chunks[1]);

    })
}

pub fn get_lines<'a>(bmarks: &'a Bookmarks, search_string: &String) -> tui::text::Text<'a> {
    if *search_string == String::from("") {
        let mut lines = vec![String::from("")];
        for bmark in &bmarks.items {
            lines.push(bmark.url.clone());
        }
        return Text::from(lines.join("\r\n"))
    }

    let mut lines: Vec<Line> = vec![];

    for bmark in &bmarks.items {
        match best_match(search_string, &bmark.url) {
            Some(value) => { 
                lines.push(Line::new(value.score(), bmark.url.clone()));
                lines.sort_by_key(|x| x.score);
            },
            None => {}
        }
    }
    let mut sorted_lines: Vec<String> = vec![];
    for line in lines {
        sorted_lines.push(line.bmark_url); 
    } 
    Text::from(sorted_lines.join("\r\n"))
}

pub struct Line {
    score: isize,
    bmark_url: String
}
impl Line {
    pub fn new(score: isize, bmark_url: String) -> Line {
        Line { score, bmark_url }
    }
}
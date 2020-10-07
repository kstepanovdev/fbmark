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

                
        let lines = get_lines(&mut app.bookmarks, &app.search_string, &mut app.filtered_bookmarks);
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Bookmarks",
                Style::default().fg(Color::White).bg(Color::Black),     
                ))
            );
        f.render_widget(input, chunks[1]);

    })
}

pub fn get_lines<'a>(bmarks: &'a mut Bookmarks, search_string: &String, filtered_bmarks: &mut Vec<Bookmark>) -> tui::text::Text<'a> {
    // TODO: try to remove dereferencing && decrease number of parameters

    if *search_string == String::from("") {
        let urls: Vec<String> = bmarks.collect_urls();
        return Text::from(urls.join("\r\n"))
    }

    let mut lines: Vec<Line> = vec![];
    for bmark in &bmarks.items {
        match best_match(search_string, &bmark.url) {
            Some(value) => { 
                lines.push(Line::new(value.score(), bmark.clone()));
            },
            None => {}
        }
    }

    lines.sort_by_key(|x| x.score);
    *filtered_bmarks = lines.into_iter().map(|x| x.bmark).collect();
    let result: Vec<String> = filtered_bmarks.iter().map(|x| x.url()).collect();
    Text::from(result.join("\r\n"))
}

pub struct Line {
    score: isize,
    bmark: Bookmark,
}
impl Line {
    pub fn new(score: isize, bmark: Bookmark) -> Line {
        Line { score, bmark }
    }
}
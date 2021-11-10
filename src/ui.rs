use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use super::app::App;
use crate::models::bookmarks::Bookmark;
use sublime_fuzzy::best_match;

pub fn draw<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(f.size());

        // search input field
        let input_string = &app.search_string;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Search", Style::default().fg(Color::White))),
        );
        f.render_widget(input, chunks[0]);

        // selection panel
        let selected_style = Style::default().fg(Color::Yellow);
        let total_bmarks_count = app.bookmarks.len().clone();
        let bmark_rows = get_lines(
            &mut app.bookmarks,
            &app.search_string,
            &mut app.filtered_bookmarks,
        );
        let filtered_bmarks_count = app.filtered_bookmarks.len();
        let bmarks_title = format!(
            "Bookmarks -- {}/{}",
            filtered_bmarks_count, total_bmarks_count
        );

        let list = List::new(bmark_rows)
            .block(Block::default().title(bmarks_title).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(selected_style)
            .highlight_symbol(">>");
        f.render_stateful_widget(list, chunks[1], &mut app.bookmarks_state);

        // add bookmark field
        let input_string = &app.new_bookmark_name;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(
            Span::styled("Add new bookmark: ", Style::default().fg(Color::White)),
        ));
        f.render_widget(input, chunks[2]);
    })
}

pub fn get_lines<'a>(
    bmarks: &'a mut Vec<Bookmark>,
    search_string: &String,
    filtered_bmarks: &mut Vec<Bookmark>,
) -> Vec<ListItem<'a>> {
    // TODO: try to remove dereferencing && decrease number of parameters

    if *search_string == String::from("") {
        *filtered_bmarks = bmarks.clone();
        return bmarks
            .iter()
            .map(|bookmark| ListItem::new(bookmark.url()))
            .collect::<Vec<ListItem>>();
    }

    let mut lines: Vec<Line> = vec![];
    for bmark in bmarks {
        match best_match(search_string, &bmark.url) {
            Some(value) => {
                lines.push(Line::new(value.score(), bmark.clone()));
            }
            None => {}
        }
    }

    lines.sort_by_key(|x| x.score);
    *filtered_bmarks = lines.into_iter().map(|x| x.bmark).collect();
    filtered_bmarks
        .into_iter()
        .map(|x| ListItem::new(x.url()))
        .collect::<Vec<ListItem>>()
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

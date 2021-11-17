use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use super::app::App;
use super::app::Mode;
use crate::models::bookmarks::Bookmark;
use sublime_fuzzy::best_match;

pub fn draw<B: Backend>(mut app: &mut App, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let selected_style = Style::default().fg(Color::Yellow);
    let default_style = Style::default().fg(Color::White);
    let is_search_mode = app.current_mode == Mode::Search;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
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
                .border_style(if is_search_mode {
                    selected_style
                } else {
                    default_style
                })
                .title(Span::styled("Search", Style::default().fg(Color::White))),
        );
        f.render_widget(input, chunks[0]);
        if is_search_mode {
            f.set_cursor(
                chunks[0].x + app.search_string.len() as u16 + 1,
                chunks[0].y + 1,
            );
        }

        // selection panel
        let total_bmarks_count = app.bookmarks.len().clone();
        let bmark_rows = get_lines(&mut app);
        let filtered_bmarks_count = app.filtered_bookmarks.len();
        let bmarks_title = format!(
            "Bookmarks -- {}/{}",
            filtered_bmarks_count, total_bmarks_count
        );

        let list = List::new(bmark_rows)
            .block(
                Block::default()
                    .title(bmarks_title)
                    .borders(Borders::ALL)
                    .border_style(if is_search_mode {
                        selected_style
                    } else {
                        default_style
                    }),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(selected_style)
            .highlight_symbol(">>");
        f.render_stateful_widget(list, chunks[1], &mut app.bookmarks_state);

        // add bookmark field
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(chunks[2]);

        let input_string = &app.new_bookmark_name;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if is_search_mode {
                    default_style
                } else {
                    selected_style
                })
                .title(Span::styled(
                    "Add new bookmark: ",
                    Style::default().fg(Color::White),
                )),
        );
        f.render_widget(input, chunks[0]);
        if !is_search_mode {
            f.set_cursor(
                chunks[0].x + app.new_bookmark_name.len() as u16 + 1,
                chunks[0].y + 1,
            );
        }

        let text = vec![
            Spans::from(Span::raw("• ~ to change modes from Input to Search and vice versa")),
            Spans::from(Span::raw("• Enter to open selected tab in xdg-defined browser")),
            Spans::from(Span::raw("• F5 to download tabs from Tagpacker")),
        ];

        let input = Paragraph::new(text)
        .block(Block::default().title("Naviagation").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

        f.render_widget(input, chunks[1]);
    })?;
    Ok(())
}

pub fn get_lines<'a>(app: &mut App) -> Vec<ListItem<'a>> {
    // TODO: try to remove dereferencing && decrease number of parameters

    if app.search_string == String::from("") {
        app.filtered_bookmarks = app.bookmarks.clone();
        return app
            .bookmarks
            .iter()
            .map(|bookmark| ListItem::new(bookmark.url()))
            .collect::<Vec<ListItem>>();
    }

    let mut lines: Vec<Line> = vec![];
    for bmark in &app.bookmarks {
        match best_match(&app.search_string, &bmark.url) {
            Some(value) => {
                lines.push(Line::new(value.score(), bmark.clone()));
            }
            None => {}
        }
    }

    lines.sort_by_key(|x| x.score);
    app.filtered_bookmarks = lines.into_iter().map(|x| x.bmark).collect();
    app.filtered_bookmarks
        .iter()
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

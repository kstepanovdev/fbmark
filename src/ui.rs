use std::io;
use tui::{
    Terminal,
    backend::Backend,
    widgets::{Widget, Block, Borders, Paragraph, Row, Table},
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
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ].as_ref()
            )
            .split(f.size());

        // search input field
        let input_string = &app.search_string;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Search",
                Style::default().fg(Color::White),
                ))
            );
        f.render_widget(input, chunks[0]);

        // selection panel
        let selected_style = Style::default().fg(Color::Yellow);
        let normal_style = Style::default().fg(Color::White);
        let bmark_rows = get_lines(&mut app.bookmarks, &app.search_string, &mut app.filtered_bookmarks);

        let mut i = 0;
        let selection_panel = bmark_rows.clone().into_iter().map(|x|
            if i == app.selected_bookmark_idx {
                i += 1;
                Row::StyledData(vec![x].into_iter(), selected_style)
            } else {
                i += 1;
                Row::StyledData(vec![x].into_iter(), normal_style)
            }
        );

        let bmarks_count: String = app.filtered_bookmarks.len().to_string();
        let highlighted_bmarks_title = ["Found bookmarks: ".to_string() + &bmarks_count];
        let t = Table::new(highlighted_bmarks_title.iter(), selection_panel)
            .block(Block::default().borders(Borders::ALL).title("Bookmarks"))
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Max(10),
            ]);

        f.render_widget(t, chunks[1]);

        // add bookmark field
        let input_string = &app.new_bookmark_name;
        let lines = Text::from((input_string).as_str());
        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                "Add new bookmark: ",
                Style::default().fg(Color::White),
                ))
            );
        f.render_widget(input, chunks[2]);
    })
}

pub fn get_lines<'a>(bmarks: &'a mut Bookmarks, search_string: &String, filtered_bmarks: &mut Vec<Bookmark>) -> Vec<String> {
    // TODO: try to remove dereferencing && decrease number of parameters

    if *search_string == String::from("") {
        *filtered_bmarks = bmarks.items.clone();
        return bmarks.collect_urls()
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
    filtered_bmarks.into_iter().map(|x| x.url()).collect()
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
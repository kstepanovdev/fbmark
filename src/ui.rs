use std::io;
use tui::{
    Terminal,
    backend::Backend,
    widgets::{Widget, Block, Borders, Paragraph, List, ListItem, Table},
    layout::{
        Layout, 
        Constraint, 
        Direction
    },
    text::{Span, Spans, Text},
    style::{Style, Color},
};

use crate::models::bookmarks::{Bookmark};
use super::app::App;
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

        // let mut i = 0;
        // let selection_panel = bmark_rows.clone().into_iter().map(|x|
        //     if i == app.selected_bookmark_idx {
        //         i += 1;
        //         Row::StyledData(vec![x].into_iter(), selected_style)
        //     } else {
        //         i += 1;
        //         Row::StyledData(vec![x].into_iter(), normal_style)
        //     }
        // );

        // let filtered_bmarks_count = app.filtered_bookmarks.len().to_string();
        // let mut total_bmarks_count = &app.bookmarks.len().to_string();
        // let highlighted_bmarks_title = [format!("{}/{}", filtered_bmarks_count, total_bmarks_count)];

        let list = List::new(bmark_rows)
            .block(Block::default().title("Bookmarks").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default())
            .highlight_symbol(">>");
        f.render_stateful_widget(list, chunks[1], &mut app.bookmarks_state);

        // TODO: refactor highlighted title
        // let filtered_bmarks_count = app.filtered_bookmarks.len().to_string();
        // let total_bmarks_count = app.bookmarks.len().to_string();
        // let highlighted_bmarks_title = [format!("{}/{}", filtered_bmarks_count, total_bmarks_count)];
        // let t = Table::new(highlighted_bmarks_title.iter(), selection_panel)
        //     .block(Block::default().borders(Borders::ALL).title("Bookmarks"))
        //     .highlight_style(selected_style)
        //     .widths(&[
        //         Constraint::Percentage(50),
        //         Constraint::Length(30),
        //         Constraint::Max(10),
        //     ]);

        // f.render_widget(t, chunks[1]);

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

pub fn get_lines<'a>(bmarks: &'a mut Vec<Bookmark>, search_string: &String, filtered_bmarks: &mut Vec<Bookmark>) -> Vec<ListItem<'a>> {
    // TODO: try to remove dereferencing && decrease number of parameters

    if *search_string == String::from("") {
        *filtered_bmarks = bmarks.clone();
        return bmarks.iter().map(|bookmark| ListItem::new(bookmark.url())).collect::<Vec<ListItem>>()

    }

    let mut lines: Vec<Line> = vec![];
    for bmark in bmarks {
        match best_match(search_string, &bmark.url) {
            Some(value) => { 
                lines.push(Line::new(value.score(), bmark.clone()));
            },
            None => {}
        }
    }

    lines.sort_by_key(|x| x.score);
    *filtered_bmarks = lines.into_iter().map(|x| x.bmark).collect();
    filtered_bmarks.into_iter().map(|x| ListItem::new(x.url())).collect::<Vec<ListItem>>()
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
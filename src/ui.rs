use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use super::app::App;
use super::app::Mode;
use crate::models::bookmarks::Bookmark;
use sublime_fuzzy::best_match;

pub struct Line {
    score: isize,
    bmark: Bookmark,
}
impl Line {
    pub fn new(score: isize, bmark: Bookmark) -> Line {
        Line { score, bmark }
    }
}

pub struct UiState {
    pub selected_style: Style,
    pub default_style: Style,
    pub is_search_mode: bool,
}

impl UiState {
    pub fn new(selected_style: Style, default_style: Style, is_search_mode: bool) -> UiState {
        UiState {
            selected_style,
            default_style,
            is_search_mode,
        }
    }
}

pub fn draw<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let ui_state = UiState::new(
        Style::default().fg(Color::Yellow),
        Style::default().fg(Color::White),
        app.current_mode == Mode::Search,
    );

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
        let search_panel = draw_search_panel(app, &ui_state);
        f.render_widget(search_panel, chunks[0]);
        if ui_state.is_search_mode {
            f.set_cursor(
                chunks[0].x + app.search_string.len() as u16 + 1,
                chunks[0].y + 1,
            );
        }

        // selection panel
        let bmark_rows = get_lines(app);
        let selection_panel = draw_selection_panel(app, bmark_rows, &ui_state);
        f.render_stateful_widget(selection_panel, chunks[1], &mut app.bookmarks_state);

        // new_bookmark field
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(chunks[2]);

        let bookmark_field = draw_bookmark_field(app, &ui_state);
        f.render_widget(bookmark_field, chunks[0]);
        if !ui_state.is_search_mode {
            f.set_cursor(
                chunks[0].x + app.new_bookmark_name.len() as u16 + 1,
                chunks[0].y + 1,
            );
        }

        // help navigation panel
        let naviagation_panel = draw_navigation_panel();
        f.render_widget(naviagation_panel, chunks[1]);
    })?;
    Ok(())
}

pub fn get_lines<'a>(app: &mut App) -> Vec<ListItem<'a>> {
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

pub fn draw_search_panel<'a>(app: &'a mut App, ui_state: &UiState) -> Paragraph<'a> {
    let input_string = &app.search_string;
    let lines = Text::from((input_string).as_str());
    Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if ui_state.is_search_mode {
                ui_state.selected_style
            } else {
                ui_state.default_style
            })
            .title(Span::styled("Search", Style::default().fg(Color::White))),
    )
}

pub fn draw_selection_panel<'a>(
    app: &mut App,
    bmark_rows: Vec<ListItem<'a>>,
    ui_state: &UiState,
) -> List<'a> {
    let total_bmarks_count = app.bookmarks.len().clone();
    let filtered_bmarks_count = app.filtered_bookmarks.len();

    let bmarks_title = format!(
        "Bookmarks -- {}/{}",
        filtered_bmarks_count, total_bmarks_count
    );

    List::new(bmark_rows)
        .block(
            Block::default()
                .title(bmarks_title)
                .borders(Borders::ALL)
                .border_style(if ui_state.is_search_mode {
                    ui_state.selected_style
                } else {
                    ui_state.default_style
                }),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(ui_state.selected_style)
        .highlight_symbol(">>")
}

pub fn draw_bookmark_field<'a>(app: &'a mut App, ui_state: &UiState) -> Paragraph<'a> {
    let input_string = &app.new_bookmark_name;
    let lines = Text::from((input_string).as_str());
    Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if ui_state.is_search_mode {
                ui_state.default_style
            } else {
                ui_state.selected_style
            })
            .title(Span::styled(
                "Add new bookmark: ",
                Style::default().fg(Color::White),
            )),
    )
}

pub fn draw_navigation_panel<'a>() -> Paragraph<'a> {
    let text = vec![
        Spans::from(Span::raw(
            "• ~ to change modes from Input to Search and vice versa",
        )),
        Spans::from(Span::raw(
            "• Enter to open selected tab in xdg-defined browser",
        )),
        Spans::from(Span::raw("• F5 to download tabs from Tagpacker")),
    ];

    Paragraph::new(text)
        .block(Block::default().title("Navigation").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
}

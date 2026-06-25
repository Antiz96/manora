//! Show a TUI menu to choose the man page from
//! Built with ratatui, inspired / based on the list widget example
//! https://ratatui.rs/examples/widgets/list/

use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListState, Paragraph};
use std::process::Command;

// Definition of the TUI modes
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Local,
    Online,
}

// State of the TUI menu (man pages list + search query)
struct App {
    items: Vec<String>,
    filtered_items: Vec<String>,
    query: String,
}

// Show the TUI menu and return the selected man page
pub fn show_menu() -> color_eyre::Result<(String, bool)> {
    color_eyre::install()?;

    let mut mode = Mode::Local;

    let items = get_man_pages()?;

    let mut app = App {
        filtered_items: items.clone(),
        items,
        query: String::new(),
    };

    let mut list_state = ListState::default().with_selected(Some(0));

    let man_selected = ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &app, &mut list_state, mode))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Down => list_state.select_next(),
                    KeyCode::Up => list_state.select_previous(),
                    KeyCode::Char(c) => {
                        app.query.push(c);
                        if mode == Mode::Local {
                            app.update_filter();
                        }
                        list_state.select(Some(0));
                    }
                    KeyCode::Backspace => {
                        app.query.pop();
                        if mode == Mode::Local {
                            app.update_filter();
                        }
                        list_state.select(Some(0));
                    }
                    KeyCode::Enter => match mode {
                        Mode::Local => {
                            if let Some(index) = list_state.selected()
                                && let Some(item) = app.filtered_items.get(index)
                            {
                                break Ok((item.clone(), false));
                            }
                        }

                        Mode::Online => {
                            let query = app.query.trim();

                            if !query.is_empty() {
                                break Ok((query.to_string(), true));
                            }
                        }
                    },
                    KeyCode::Tab => {
                        mode = match mode {
                            Mode::Local => Mode::Online,
                            Mode::Online => Mode::Local,
                        };
                    }
                    KeyCode::Esc => {
                        break Err(color_eyre::eyre::eyre!("No man page selected"));
                    }
                    _ => {}
                }
            }
        }
    })?;

    Ok(man_selected)
}

// Get the list of local man pages
fn get_man_pages() -> color_eyre::Result<Vec<String>> {
    let man_list = Command::new("man").args(["-k", "."]).output()?;

    Ok(String::from_utf8_lossy(&man_list.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect())
}

// Render the man page list
fn render_man_page_list(
    frame: &mut Frame,
    area: Rect,
    items: &[String],
    list_state: &mut ListState,
) {
    let list = List::new(items.iter().map(|item| item.as_str()))
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

// Render the helper footer text
fn render_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(
        "Navigate with arrow keys, type to search, select with Enter, switch mode with TAB, exit with ESC",
    );

    frame.render_widget(help.centered(), area);
}

// Implementation of the search / filtering functionality
// Update the displayed list according to search queries
impl App {
    fn update_filter(&mut self) {
        let query = self.query.to_lowercase();

        let mut exact = Vec::new();
        let mut fuzzy = Vec::new();

        for item in &self.items {
            let item_lower = item.to_lowercase();

            if item_lower == query {
                exact.push(item.clone());
            } else if matches_query(&item_lower, &query) {
                fuzzy.push(item.clone());
            }
        }

        exact.extend(fuzzy);

        self.filtered_items = exact;
    }
}

// Filter results according to search queries
fn matches_query(item: &str, query: &str) -> bool {
    let mut chars = query.chars();

    item.chars()
        .filter(|c| chars.next().is_some_and(|q| *c == q))
        .count()
        == query.len()
}

// Render the TUI interface with the different lists
fn render(frame: &mut Frame, app: &App, list_state: &mut ListState, mode: Mode) {
    let constraints = [
        Constraint::Length(1), // header
        Constraint::Length(2), // mode description
        Constraint::Length(1), // search
        Constraint::Fill(1),   // list
        Constraint::Length(1), // footer
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [header, mode_desc_area, search_area, list, footer] = frame.area().layout(&layout);

    let title = match mode {
        Mode::Local => Line::from("[Local] Online").bold(),
        Mode::Online => Line::from(" Local [Online]").bold(),
    };
    frame.render_widget(title.centered(), header);

    let mode_desc = match mode {
        Mode::Local => Text::from(vec![
            Line::from("Local mode:"),
            Line::from("Search a local man page to open"),
        ]),
        Mode::Online => Text::from(vec![
            Line::from("Online mode:"),
            Line::from("Type a man page name to download from https://manned.org"),
        ]),
    };
    frame.render_widget(Paragraph::new(mode_desc).centered(), mode_desc_area);

    let search = Paragraph::new(Line::from(vec![
        Span::styled("Search: ", ratatui::style::Style::default().bold()),
        Span::raw(&app.query),
    ]));
    frame.render_widget(search, search_area);

    if mode == Mode::Local {
        render_man_page_list(frame, list, &app.filtered_items, list_state);
    }

    render_help(frame, footer);
}

//! Show a TUI menu to choose the man page from
//! Built with ratatui, inspired / based on the list widget example
//! https://ratatui.rs/examples/widgets/list/

// Import external modules
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{List, ListState, Paragraph};
use std::process::Command;

// State of the TUI menu (man pages list + search query)
struct App {
    items: Vec<String>,
    filtered_items: Vec<String>,
    query: String,
}

// Show the TUI menu and return the selected man page
pub fn show_menu() -> color_eyre::Result<String> {
    color_eyre::install()?;

    let items = get_man_pages()?;

    let mut app = App {
        filtered_items: items.clone(),
        items,
        query: String::new(),
    };

    let mut list_state = ListState::default().with_selected(Some(0));

    let man_selected = ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &app, &mut list_state))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Down => list_state.select_next(),
                    KeyCode::Up => list_state.select_previous(),
                    KeyCode::Char(c) => {
                        app.query.push(c);
                        app.update_filter();
                        list_state.select(Some(0));
                    }
                    KeyCode::Backspace => {
                        app.query.pop();
                        app.update_filter();
                        list_state.select(Some(0));
                    }
                    KeyCode::Enter => {
                        if let Some(index) = list_state.selected()
                            && let Some(item) = app.filtered_items.get(index)
                        {
                            break Ok(item.clone());
                        }
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
    let output = Command::new("man").args(["-k", "."]).output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect())
}

// Render the man page list
fn render_man_page_list(frame: &mut Frame, area: Rect, items: &[String], list_state: &mut ListState) {
    let list = List::new(items.iter().map(|item| item.as_str()))
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

// Render the helper footer text
fn render_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(
        "Navigate with arrow keys, type to search, select with Enter, exit with ESC",
    );

    frame.render_widget(help.centered(), area);
}

// Implementation of the search / filtering functionality
// Update the displayed list according to the search query
impl App {
    fn update_filter(&mut self) {
        let query = self.query.to_lowercase();

        self.filtered_items = self
            .items
            .iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .cloned()
            .collect();
    }
}

// Render the TUI interface with the different lists
fn render(frame: &mut Frame, app: &App, list_state: &mut ListState) {
    let constraints = [
        Constraint::Length(1), // header
        Constraint::Length(1), // search
        Constraint::Fill(1),   // list
        Constraint::Length(1), // footer
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [header, search_area, list, footer] = frame.area().layout(&layout);

    let title = Line::from("Man Pages").bold();
    frame.render_widget(title.centered(), header);

    let search = Paragraph::new(format!("Search: {}", app.query));
    frame.render_widget(search, search_area);

    render_man_page_list(frame, list, &app.filtered_items, list_state);
    render_help(frame, footer);
}

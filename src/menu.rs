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

// Show the TUI menu
pub fn show_menu() -> color_eyre::Result<String> {
    color_eyre::install()?;

    let items = get_man_pages()?;

    let mut list_state = ListState::default().with_selected(Some(0));

    let man_selected = ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &items, &mut list_state))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Down => list_state.select_next(),
                    KeyCode::Up => list_state.select_previous(),
                    KeyCode::Enter => {
                        if let Some(index) = list_state.selected() {
                            break Ok(items[index].clone());
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

// Render the UI with lists
fn render(frame: &mut Frame, items: &[String], list_state: &mut ListState) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [header, list, footer] = frame.area().layout(&layout);

    let title = Line::from("Man Pages").bold();
    frame.render_widget(title.centered(), header);

    render_list(frame, list, items, list_state);
    render_help(frame, footer);
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

// Render list
fn render_list(frame: &mut Frame, area: Rect, items: &[String], list_state: &mut ListState) {
    let list = List::new(items.iter().map(|item| item.as_str()))
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

// Helper footer text
fn render_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(
        "Navigate with arrow keys, type to search, select with Enter, exit with ESC",
    );

    frame.render_widget(help.centered(), area);
}

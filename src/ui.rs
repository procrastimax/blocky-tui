use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

// TODO: replace App with Model struct? -> App does not contain complete data? -> or should the App
// struct be modeled to contain all important data?
pub fn render(_app: &mut App, frame: &mut Frame) {
    // tiles are the individual layout components
    let main_tiles = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10), // Title Block
            Constraint::Percentage(80), // Main Tiles
            Constraint::Percentage(10), // Bottom Tile
        ])
        .split(frame.size());

    let app_tiles = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(main_tiles[1]);

    render_title(_app, main_tiles[0], frame);

    render_dns_status_tile(_app, app_tiles[0], frame);
    render_blocking_status_tile(_app, app_tiles[1], frame);
    render_refresh_list_tile(_app, app_tiles[2], frame);

    render_query_tile(_app, main_tiles[2], frame);
}

fn render_title(_app: &mut App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Blocky TUI",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

fn render_query_tile(_app: &mut App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Hier könnte Ihre Query stehen",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

fn render_dns_status_tile(_app: &mut App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "DNS STATUS",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

fn render_blocking_status_tile(_app: &mut App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Blocking STATUS",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

fn render_refresh_list_tile(_app: &mut App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Refresh Lists",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

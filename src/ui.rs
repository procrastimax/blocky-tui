use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::{App, CurrentFocus};

pub fn render(_app: &App, frame: &mut Frame) {
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

fn render_dns_status_tile(app: &App, r: Rect, frame: &mut Frame) {
    let tile_block = if matches!(app.current_focus, CurrentFocus::DNSStatus) {
        get_focused_tile("DNS Status")
    } else {
        get_unfocused_tile("DNS Status")
    };

    frame.render_widget(tile_block, r);
}

fn render_blocking_status_tile(app: &App, r: Rect, frame: &mut Frame) {
    let tile_block = if matches!(app.current_focus, CurrentFocus::BlockingStatus) {
        get_focused_tile("Blocking Status")
    } else {
        get_unfocused_tile("Blocking Status")
    };

    frame.render_widget(tile_block, r);
}

fn render_refresh_list_tile(app: &App, r: Rect, frame: &mut Frame) {
    let tile_block = if matches!(app.current_focus, CurrentFocus::RefreshLists) {
        get_focused_tile("Refresh Lists")
    } else {
        get_unfocused_tile("Refresh Lists")
    };

    frame.render_widget(tile_block, r);
}

fn render_query_tile(app: &App, r: Rect, frame: &mut Frame) {
    let tile_block = if matches!(app.current_focus, CurrentFocus::QueryDNS) {
        get_focused_tile("Query")
    } else {
        get_unfocused_tile("Query")
    };

    frame.render_widget(tile_block, r);
}

fn render_title(_app: &App, r: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Blocky TUI",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(title, r);
}

fn get_focused_tile(title: &str) -> Paragraph<'_> {
    Paragraph::new(Text::styled(title, Style::default().fg(Color::Yellow)))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Thick),
        )
        .bold()
}

fn get_unfocused_tile(title: &str) -> Paragraph<'_> {
    Paragraph::new(Text::styled(title, Style::default().fg(Color::White)))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded),
        )
}

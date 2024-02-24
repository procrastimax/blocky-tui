use std::borrow::Borrow;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::{App, BlockingStatus, CurrentFocus, DNSStatus};

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
    let status_line = {
        match app.dns_status {
            Some(DNSStatus::Healthy) => vec![
                Line::from(Span::styled("Healthy", Style::default().fg(Color::Green))),
                Line::from(Span::from("DNS server is reachable and healty")),
            ],
            Some(DNSStatus::Unhealthy) => {
                vec![
                    Line::from(Span::styled(
                        "Unhealthy",
                        Style::default().fg(Color::Magenta),
                    )),
                    Line::from(Span::from("DNS server is reachable but unhealthy")),
                ]
            }
            Some(DNSStatus::NoResponse) => {
                vec![
                    Line::from(Span::styled("No Response", Style::default().fg(Color::Red))),
                    Line::from(Span::from("DNS server is not reachable")),
                ]
            }
            None => vec![
                Line::from(""),
                Line::styled("Not queried", Style::default().fg(Color::White).bold()),
                Line::styled(
                    "DNS status is not set",
                    Style::default().fg(Color::White).italic(),
                ),
            ],
        }
    };

    let status_par = Paragraph::new(status_line)
        .centered()
        .block(get_block(app, CurrentFocus::DNSStatus).title("DNS Status"));

    frame.render_widget(status_par, r);
}

fn render_blocking_status_tile(app: &App, r: Rect, frame: &mut Frame) {
    let blocking_line = {
        match &app.blocking_status {
            Some(status) => {
                if status.is_blocking_enabled {
                    vec![
                        Line::from(Span::styled("Blocking", Style::default().fg(Color::Green))),
                        Line::from(Span::from("DNS server is currently blocking")),
                    ]
                } else {
                    vec![
                        Line::from(Span::styled(
                            "Not Blocking",
                            Style::default().fg(Color::Green),
                        )),
                        Line::from(Span::from("DNS server is not blocking")),
                    ]
                }
            }
            None => vec![
                Line::from(""),
                Line::styled("Not queried", Style::default().fg(Color::White).bold()),
                Line::styled(
                    "Blocking status is not set",
                    Style::default().fg(Color::White).italic(),
                ),
            ],
        }
    };

    let blocking_par = Paragraph::new(blocking_line)
        .centered()
        .block(get_block(app, CurrentFocus::BlockingStatus).title("Blocking Status"));

    frame.render_widget(blocking_par, r);
}

fn render_refresh_list_tile(app: &App, r: Rect, frame: &mut Frame) {
    let lists_par = Paragraph::new("Refresh Blocking Lists")
        .centered()
        .block(get_block(app, CurrentFocus::RefreshLists).title("Refresh Lists"));

    frame.render_widget(lists_par, r);
}

fn render_query_tile(app: &App, r: Rect, frame: &mut Frame) {
    let query_par = Paragraph::new("Query DNS")
        .centered()
        .block(get_block(app, CurrentFocus::QueryDNS).title("Query DNS"));

    frame.render_widget(query_par, r);
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

fn get_block(app: &App, tile: CurrentFocus) -> Block<'_> {
    if app.current_focus == tile {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Thick)
    } else {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
    }
}

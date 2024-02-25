use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::{App, CurrentFocus, DNSStatus};

pub fn render(_app: &App, frame: &mut Frame) {
    // tiles are the individual layout components
    let main_tiles = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20), // Title Block
            Constraint::Percentage(45), // Main Tiles
            Constraint::Percentage(35), // Bottom Tile
        ])
        .split(frame.size());

    let mid_tiles = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(main_tiles[1]);

    let bottom_tiles = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_tiles[2]);

    render_title(_app, main_tiles[0], frame);

    render_dns_status_tile(_app, mid_tiles[0], frame);
    render_blocking_status_tile(_app, mid_tiles[1], frame);
    render_refresh_list_tile(_app, mid_tiles[2], frame);

    render_query_tile(_app, bottom_tiles[0], frame);
    render_cache_delete_tile(_app, bottom_tiles[1], frame);
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
                Line::styled("Not queried", Style::default().fg(Color::White).bold()),
                Line::styled(
                    "DNS status is not set",
                    Style::default().fg(Color::White).italic(),
                ),
            ],
        }
    };

    let status_par = Paragraph::new(status_line).centered().block(get_block(
        app,
        CurrentFocus::DNSStatus,
        format!("[{}] Delete Cache", CurrentFocus::DNSStatus as u8),
    ));

    frame.render_widget(status_par, r);
}

fn render_blocking_status_tile(app: &App, r: Rect, frame: &mut Frame) {
    // TODO: render seconds and group names if blocking is disabled
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
                Line::styled("Not queried", Style::default().fg(Color::White).bold()),
                Line::styled(
                    "Blocking status is not set",
                    Style::default().fg(Color::White).italic(),
                ),
            ],
        }
    };

    let blocking_par = Paragraph::new(blocking_line).centered().block(get_block(
        app,
        CurrentFocus::BlockingStatus,
        format!("[{}] Delete Cache", CurrentFocus::BlockingStatus as u8),
    ));

    frame.render_widget(blocking_par, r);
}

fn render_refresh_list_tile(app: &App, r: Rect, frame: &mut Frame) {
    let lists_par = Paragraph::new("Refresh Blocking Lists")
        .centered()
        .block(get_block(
            app,
            CurrentFocus::RefreshLists,
            format!("[{}] Delete Cache", CurrentFocus::RefreshLists as u8),
        ));

    frame.render_widget(lists_par, r);
}

fn render_query_tile(app: &App, r: Rect, frame: &mut Frame) {
    let query_par = Paragraph::new("Query DNS").centered().block(get_block(
        app,
        CurrentFocus::QueryDNS,
        format!("[{}] Delete Cache", CurrentFocus::QueryDNS as u8),
    ));

    frame.render_widget(query_par, r);
}

fn render_cache_delete_tile(app: &App, r: Rect, frame: &mut Frame) {
    let query_par = Paragraph::new("Delete Cache").centered().block(get_block(
        app,
        CurrentFocus::DeleteCache,
        format!("[{}] Delete Cache", CurrentFocus::DeleteCache as u8),
    ));

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

fn get_block(app: &App, tile: CurrentFocus, block_title: String) -> Block<'_> {
    if app.current_focus == tile {
        let title = Span::styled(block_title, Style::default().bold());
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Thick)
            .title(title)
            .padding(Padding::uniform(1))
    } else {
        let title = Span::styled(block_title, Style::default());
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1))
            .title(title)
    }
}

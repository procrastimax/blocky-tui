use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentFocus, DNSStatus};

impl App {
    pub fn render(&self, frame: &mut Frame) {
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

        self.render_title(main_tiles[0], frame);

        self.render_dns_status_tile(mid_tiles[0], frame);
        self.render_blocking_status_tile(mid_tiles[1], frame);
        self.render_refresh_list_tile(mid_tiles[2], frame);

        self.render_query_tile(bottom_tiles[0], frame);
        self.render_cache_delete_tile(bottom_tiles[1], frame);
    }

    fn render_dns_status_tile(&self, r: Rect, frame: &mut Frame) {
        let status_line = {
            match self.dns_status {
                Some(DNSStatus::Healthy) => vec![
                    Line::from(Span::styled("Healthy", Style::default().fg(Color::Green))),
                    Line::from(Span::from("DNS is reachable and healty")),
                ],
                Some(DNSStatus::Unhealthy) => {
                    vec![
                        Line::from(Span::styled(
                            "Unhealthy",
                            Style::default().fg(Color::Magenta),
                        )),
                        Line::from(Span::from("DNS is reachable but unhealthy")),
                    ]
                }
                Some(DNSStatus::NoResponse) => {
                    vec![
                        Line::from(Span::styled("No Response", Style::default().fg(Color::Red))),
                        Line::styled(
                            "DNS is not reachable",
                            Style::default().fg(Color::White).italic(),
                        ),
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

        let status_details = {
            vec![
                Line::from("- [] DNS Domain is reachable"),
                Line::from("- [] DNS Port is open"),
                Line::from("- [] Received answer from DNS Server"),
            ]
        };

        let block = self.get_block(
            CurrentFocus::DNSStatus,
            format!("[{}] Delete Cache", CurrentFocus::DNSStatus as u8),
        );
        let split_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(block.inner(r));
        frame.render_widget(block, r);

        let status_par = Paragraph::new(status_line)
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(status_par, split_layout[0]);

        let area = self.centered_rect(50, 99, split_layout[1]);
        let details_par = Paragraph::new(status_details)
            .left_aligned()
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));
        frame.render_widget(details_par, area);
    }

    fn render_blocking_status_tile(&self, r: Rect, frame: &mut Frame) {
        // TODO: render seconds and group names if blocking is disabled
        let blocking_line = {
            match &self.blocking_status {
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

        let blocking_par = Paragraph::new(blocking_line)
            .centered()
            .block(self.get_block(
                CurrentFocus::BlockingStatus,
                format!("[{}] Delete Cache", CurrentFocus::BlockingStatus as u8),
            ));

        frame.render_widget(blocking_par, r);
    }

    fn render_refresh_list_tile(&self, r: Rect, frame: &mut Frame) {
        let lists_par = Paragraph::new("Refresh Blocking Lists")
            .centered()
            .block(self.get_block(
                CurrentFocus::RefreshLists,
                format!("[{}] Delete Cache", CurrentFocus::RefreshLists as u8),
            ));

        frame.render_widget(lists_par, r);
    }

    fn render_query_tile(&self, r: Rect, frame: &mut Frame) {
        let query_par = Paragraph::new("Query DNS").centered().block(self.get_block(
            CurrentFocus::QueryDNS,
            format!("[{}] Delete Cache", CurrentFocus::QueryDNS as u8),
        ));

        frame.render_widget(query_par, r);
    }

    fn render_cache_delete_tile(&self, r: Rect, frame: &mut Frame) {
        let query_par = Paragraph::new("Delete Cache")
            .centered()
            .block(self.get_block(
                CurrentFocus::DeleteCache,
                format!("[{}] Delete Cache", CurrentFocus::DeleteCache as u8),
            ));

        frame.render_widget(query_par, r);
    }

    fn render_title(&self, r: Rect, frame: &mut Frame) {
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

    fn get_block(&self, tile: CurrentFocus, block_title: String) -> Block<'_> {
        if self.current_focus == tile {
            let title = Span::styled(block_title, Style::default().bold());
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Thick)
                .title(title)
                .padding(Padding::uniform(3))
        } else {
            let title = Span::styled(block_title, Style::default());
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(3))
                .title(title)
        }
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}

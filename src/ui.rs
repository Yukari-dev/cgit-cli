use std::fmt::format;

use crate::app::{App, CurrentScreen, RepoItem};
use ratatui::{prelude::*, widgets::*};

const LOGO: &str = r#"
 _______  _______ __________________
(  ____ \(  ____ \\__   __/\__   __/
| (    \/| (    \/   ) (      ) (   
| |      | |         | |      | |   
| |      | | ____    | |      | |   
| |      | | \_  )   | |      | |   
| (____/\| (___) |___) (___   | |   
(_______/(_______)\_______/   )_(   
                                                                                                                                                         
"#;

const THEME_PURPLE: Color = Color::Rgb(153, 41, 234);
const THEME_MAGENTA: Color = Color::Rgb(255, 0, 255);

pub fn render(f: &mut Frame, app: &App) {
    // 1. FIX: Always render background first so the whole terminal is covered
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(10, 10, 20))),
        f.size(),
    );
    let area = centered_rect(80, 50, f.size()); // Changed from 100 to 40 to actually center it

    match app.screen {
        CurrentScreen::Input => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(9), Constraint::Length(3)])
                .split(area);

            f.render_widget(
                Paragraph::new(LOGO)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(THEME_MAGENTA)),
                chunks[0],
            );

            let input_title = if app.loading {
                " [ FETCHING CONTENT... ] "
            } else {
                " [ Repository URL ] "
            };

            f.render_widget(
                Paragraph::new(app.input_buffer.as_str())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(THEME_PURPLE))
                            .title(input_title),
                    )
                    .style(Style::default().fg(Color::White)),
                chunks[1],
            );

            f.set_cursor(
                chunks[1].x + app.cursor_position as u16 + 1,
                chunks[1].y + 1,
            );
        }
        CurrentScreen::Loading => {
            // 2. Futuristic Loading Screen
            let area = centered_rect(40, 10, f.size());
            let loading_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_MAGENTA))
                .title(" SYSTEM STATUS ");

            let loading_text = Paragraph::new("INITIALIZING CONNECTION...")
                .alignment(Alignment::Center)
                .block(loading_block)
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(loading_text, area);
        }
        CurrentScreen::FileList => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.size());
            let path = format!(" REPO: {} / {} / {}", app.owner, app.repo, app.current_path);
            let header = Paragraph::new(path).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(THEME_MAGENTA))
                    .style(Style::default().fg(Color::White)),
            );
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = app
                .items
                .iter()
                .map(|i| {
                    let icon = if i.is_dir { " [DIR] " } else { " [FILE] " };
                    ListItem::new(format!("{} {}", icon, i.name)).style(
                        Style::default().fg(if i.is_dir { THEME_PURPLE } else { Color::White }),
                    )
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
                .highlight_style(
                    Style::default()
                        .bg(THEME_MAGENTA)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" >> ");
            f.render_stateful_widget(list, chunks[1], &mut app.list_state.clone());

            let footer =
                Paragraph::new(" [ENTER] Open/Download [SPACE] Mark [ESCAPE] Back [q] Quit")
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(THEME_PURPLE)),
                    )
                    .alignment(Alignment::Center);
            f.render_widget(footer, chunks[2]);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

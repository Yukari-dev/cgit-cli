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

const THEME_BG: Color = Color::Rgb(15, 17, 23);
const THEME_PRIMARY: Color = Color::Rgb(136, 192, 208);
const THEME_ACCENT: Color = Color::Rgb(163, 190, 140);
const THEME_DIM: Color = Color::Rgb(76, 86, 106);

pub fn render(f: &mut Frame, app: &App) {
    // 1. FIX: Always render background first so the whole terminal is covered
    f.render_widget(
        Block::default().style(Style::default().bg(THEME_BG)),
        f.size(),
    );
    let area = centered_rect(80, 50, f.size()); // Changed from 100 to 40 to actually center it

    match app.screen {
        CurrentScreen::Input => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(9),
                    Constraint::Length(2),
                    Constraint::Length(3),
                ])
                .split(area);

            f.render_widget(
                Paragraph::new(LOGO)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(THEME_PRIMARY)),
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
                            .border_style(Style::default().fg(THEME_DIM))
                            .title(Span::styled(input_title, Style::default().fg(THEME_ACCENT))),
                    )
                    .style(Style::default().fg(Color::White)),
                chunks[2],
            );

            f.set_cursor(
                chunks[2].x + app.cursor_position as u16 + 1,
                chunks[2].y + 1,
            );
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
            let display_path = app.current_path.to_string_lossy();
            let header_text = format!("  {}/{}/{}", app.owner, app.repo, display_path);
            let header = Paragraph::new(header_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(THEME_DIM))
                    .style(Style::default().fg(Color::White)),
            );
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = app
                .items
                .iter()
                .map(|i| {
                    let is_marked = app.marked_paths.contains(&i.path);

                    let mark_icon = if is_marked { "󰄲 " } else { "  " };
                    let type_icon = if i.is_dir { " " } else { " " };

                    let content = format!("{}{} {}", mark_icon, type_icon, i.name);
                    let style = if is_marked {
                        Style::default()
                            .fg(THEME_ACCENT)
                            .add_modifier(Modifier::BOLD)
                    } else if i.is_dir {
                        Style::default().fg(THEME_PRIMARY)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(content).style(style)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
                .highlight_style(
                    Style::default()
                        .bg(THEME_DIM)
                        .fg(THEME_ACCENT)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" [  ] ");
            f.render_stateful_widget(list, chunks[1], &mut app.list_state.clone());

            let footer_text = format!(
                " [ENTER] Open | [SPACE] Mark ({}) | [ESC] Back | [Q] Quit ",
                app.marked_paths.len()
            );
            f.render_widget(
                Paragraph::new(footer_text)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(THEME_DIM)),
                chunks[2],
            );
        }
        CurrentScreen::Loading => {
            // 2. Futuristic Loading Screen
            let area = centered_rect(40, 10, f.size());
            let loading_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_ACCENT))
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

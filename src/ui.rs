use crate::app::{App, CurrentScreen};
use ratatui::{prelude::*, widgets::*};

// EDEX-UI Color Palette
const BG: Color = Color::Rgb(0, 0, 0);
const CYAN: Color = Color::Rgb(0, 255, 255);
const CYAN_DIM: Color = Color::Rgb(0, 140, 140);
const CYAN_DARK: Color = Color::Rgb(0, 60, 60);
const BLUE: Color = Color::Rgb(0, 120, 255);
const BLUE_DIM: Color = Color::Rgb(0, 60, 120);
const WHITE: Color = Color::Rgb(220, 220, 220);
const DIM: Color = Color::Rgb(50, 50, 50);
const GREEN: Color = Color::Rgb(0, 255, 140);
const RED: Color = Color::Rgb(255, 60, 60);

const LOGO: &str = concat!(
    " ██████╗ ██████╗ ██╗████████╗\n",
    "██╔════╝██╔════╝ ██║╚══██╔══╝\n",
    "██║     ██║  ███╗██║   ██║   \n",
    "██║     ██║   ██║██║   ██║   \n",
    "╚██████╗╚██████╔╝██║   ██║   \n",
    " ╚═════╝ ╚═════╝ ╚═╝   ╚═╝   \n",
);

const TAGLINE: &str = "[ GITHUB SURGICAL EXTRACTION TOOL ]";

pub fn render(f: &mut Frame, app: &App) {
    f.render_widget(Block::default().style(Style::default().bg(BG)), f.size());

    match app.screen {
        CurrentScreen::Input => render_input(f, app),
        CurrentScreen::FileList => render_file_list(f, app),
        CurrentScreen::Loading => render_loading(f, app),
    }
}

fn render_input(f: &mut Frame, app: &App) {
    let area = f.size();

    // outer border
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(CYAN_DIM))
            .title(Span::styled(
                "╡ CGIT // NEURAL LINK ESTABLISHED ╞",
                Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(BG)),
        area,
    );

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Percentage(20),
        ])
        .margin(2)
        .split(area);

    // logo
    f.render_widget(
        Paragraph::new(LOGO)
            .alignment(Alignment::Center)
            .style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
        inner[1],
    );

    // tagline
    f.render_widget(
        Paragraph::new(TAGLINE)
            .alignment(Alignment::Center)
            .style(Style::default().fg(CYAN_DIM)),
        inner[3],
    );

    // divider
    f.render_widget(
        Paragraph::new("─".repeat(area.width as usize / 2))
            .alignment(Alignment::Center)
            .style(Style::default().fg(DIM)),
        inner[4],
    );

    // input box
    let input_label = if app.loading {
        "⟨ SCANNING REPOSITORY... ⟩"
    } else {
        "⟨ ENTER TARGET REPOSITORY ⟩"
    };

    let input_color = if app.loading { BLUE } else { CYAN };

    // center the input box horizontally
    let input_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(inner[5]);

    f.render_widget(
        Paragraph::new(app.input_buffer.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(input_color))
                    .title(Span::styled(
                        input_label,
                        Style::default()
                            .fg(input_color)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(WHITE).bg(BG)),
        input_row[1],
    );

    f.set_cursor(
        input_row[1].x + app.cursor_position as u16 + 1,
        input_row[1].y + 1,
    );

    // hint
    f.render_widget(
        Paragraph::new("e.g.  owner/repo  or  https://github.com/owner/repo")
            .alignment(Alignment::Center)
            .style(Style::default().fg(DIM)),
        inner[7],
    );
}

fn render_file_list(f: &mut Frame, app: &App) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // header
    let display_path = app.current_path.to_string_lossy();
    let path_display = if display_path.is_empty() {
        format!(" ⟨ {}/{} ⟩ ", app.owner, app.repo)
    } else {
        format!(" ⟨ {}/{}/{} ⟩ ", app.owner, app.repo, display_path)
    };

    f.render_widget(
        Paragraph::new(path_display.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(CYAN_DIM))
                    .title(Span::styled(
                        " CGIT // FILE SYSTEM ",
                        Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
                    ))
                    .title_alignment(Alignment::Left)
                    .title_bottom(Span::styled(
                        format!(" {} ITEMS ", app.items.len()),
                        Style::default().fg(CYAN_DIM),
                    ))
                    .style(Style::default().bg(BG)),
            )
            .style(Style::default().fg(WHITE))
            .alignment(Alignment::Center),
        chunks[0],
    );

    // file list
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| {
            let is_marked = app.marked_paths.contains(&i.path);
            let mark = if is_marked { "◆" } else { "◇" };
            let icon = if i.is_dir { "▶ " } else { "  " };
            let content = format!(" {} {}  {}", mark, icon, i.name);

            let style = if is_marked {
                Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
            } else if i.is_dir {
                Style::default().fg(CYAN)
            } else {
                Style::default().fg(WHITE)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(DIM))
                .style(Style::default().bg(BG)),
        )
        .highlight_style(
            Style::default()
                .bg(CYAN_DARK)
                .fg(CYAN)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶▶ ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state.clone());

    // footer
    let marked_count = app.marked_paths.len();
    let marked_color = if marked_count > 0 { GREEN } else { DIM };

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[2]);

    let keys = [
        ("[ENTER]", "OPEN", CYAN),
        ("[SPACE]", "MARK", marked_color),
        (
            "[D]",
            "DOWNLOAD",
            if marked_count > 0 { GREEN } else { DIM },
        ),
        ("[ESC]", "BACK", BLUE),
    ];

    for (i, (key, label, color)) in keys.iter().enumerate() {
        f.render_widget(
            Paragraph::new(format!(" {} {} ", key, label))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(*color)),
                )
                .style(Style::default().fg(*color).add_modifier(Modifier::BOLD)),
            footer_chunks[i],
        );
    }
}

fn render_loading(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.size());

    f.render_widget(
        Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "◆◆◆  EXTRACTING DATA  ◆◆◆",
                Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "CONNECTING TO GITHUB NEURAL NETWORK...",
                Style::default().fg(CYAN_DIM),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(CYAN))
                .title(Span::styled(
                    " SYSTEM STATUS ",
                    Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(BG)),
        ),
        area,
    );
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

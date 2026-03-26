use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::{App, CurrentScreen};
use ratatui::{prelude::*, widgets::*};

const BG: Color = Color::Rgb(25, 23, 36); // Ebony
const CYAN: Color = Color::Rgb(156, 207, 216); // Foam (Teal-ish)
const CYAN_DIM: Color = Color::Rgb(62, 143, 176);
const CYAN_DARK: Color = Color::Rgb(40, 37, 55);
const BLUE: Color = Color::Rgb(196, 167, 231); // Iris (Lavender)
const BLUE_DIM: Color = Color::Rgb(144, 122, 169);
const WHITE: Color = Color::Rgb(224, 222, 244); // Off-White
const DIM: Color = Color::Rgb(110, 106, 134); // Muted Purple-Grey
const GREEN: Color = Color::Rgb(235, 188, 186); // Rose (Muted Pink)
const RED: Color = Color::Rgb(235, 111, 111); // Love (Warm Red)

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

    // Background Grid/Decorative Lines (Minimalist futuristic touch)
    f.render_widget(
        Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(CYAN_DARK))
            .title(Span::styled(" v1.0.4 ", Style::default().fg(DIM))),
        area,
    );

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(7), // Logo
            Constraint::Length(2), // Spacer
            Constraint::Length(1), // Tagline
            Constraint::Length(4), // Spacer
            Constraint::Length(3), // Input Box
            Constraint::Percentage(25),
        ])
        .split(area);

    // logo - using CYAN for the primary "glow"
    f.render_widget(
        Paragraph::new(LOGO)
            .alignment(Alignment::Center)
            .style(Style::default().fg(CYAN)),
        inner[1],
    );

    f.render_widget(
        Paragraph::new(TAGLINE)
            .alignment(Alignment::Center)
            .style(Style::default().fg(CYAN_DIM).add_modifier(Modifier::ITALIC)),
        inner[3],
    );

    // Input area - Minimalist "Scanning" bar
    let input_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(inner[5]);

    let input_block = Block::default()
        .borders(Borders::BOTTOM) // Only bottom border for a "line input" feel
        .border_style(Style::default().fg(if app.loading { BLUE } else { CYAN_DIM }))
        .title(Span::styled(
            if app.loading {
                " SCANNING... "
            } else {
                " IDENTIFY TARGET "
            },
            Style::default().fg(DIM),
        ));

    f.render_widget(
        Paragraph::new(app.input_buffer.as_str())
            .block(input_block)
            .style(Style::default().fg(WHITE)),
        input_row[1],
    );

    f.set_cursor(
        input_row[1].x + app.cursor_position as u16,
        input_row[1].y + 1,
    );
}

fn render_file_list(f: &mut Frame, app: &App) {
    let area = f.size();

    // Main layout with vertical padding for that "contained" look
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Header
            Constraint::Min(0),    // Workspace
            Constraint::Length(1), // Log Bar
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // --- 1. HEADER ---
    let header_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0]);

    f.render_widget(
        Paragraph::new(format!("   {}/{}", app.owner, app.repo))
            .style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
        header_area[0],
    );

    f.render_widget(
        Paragraph::new(format!("NODE: {} ", app.current_path.to_string_lossy()))
            .alignment(Alignment::Right)
            .style(Style::default().fg(CYAN_DIM)),
        header_area[1],
    );

    // --- 2. WORKSPACE (THE "THREE COLUMN" FIX) ---
    let workspace = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12), // Left decorative "Nav" bar
            Constraint::Min(0),     // Center File List
            Constraint::Length(24), // Right "Telemetry" bar
        ])
        .split(chunks[1]);

    // A. Left Decorative "Nav" (Minimalist vertical IDs)
    let left_decor = vec![
        Line::from(Span::styled("ID_01", Style::default().fg(CYAN_DARK))),
        Line::from(Span::styled("ID_02", Style::default().fg(CYAN_DARK))),
        Line::from(Span::styled("------", Style::default().fg(DIM))),
        Line::from(Span::styled("SRG_EXT", Style::default().fg(BLUE))),
    ];
    f.render_widget(
        Paragraph::new(left_decor).block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(CYAN_DARK)),
        ),
        workspace[0],
    );

    // B. Center File List (The Blade)
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| {
            let is_marked = app.marked_paths.contains(&i.path);
            let prefix = if is_marked { "▐" } else { " " };
            let icon = if i.is_dir { " " } else { " " };
            let style = if is_marked {
                Style::default().fg(GREEN).bg(Color::Rgb(20, 0, 10))
            } else if i.is_dir {
                Style::default().fg(CYAN)
            } else {
                Style::default().fg(WHITE)
            };
            ListItem::new(format!("{} {} {}", prefix, icon, i.name)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(CYAN_DARK)),
        )
        .highlight_style(Style::default().bg(CYAN_DARK).fg(WHITE))
        .highlight_symbol("❯ ");
    f.render_stateful_widget(list, workspace[1], &mut app.list_state.clone());

    // C. Right Telemetry (Live Hex Stream)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut telemetry = vec![
        Line::from(Span::styled(
            " TELEMETRY ",
            Style::default().bg(CYAN_DARK).fg(WHITE),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(" MARKED: {:0>2}", app.marked_paths.len()),
            Style::default().fg(GREEN),
        )),
        Line::from(Span::styled(
            format!(" TOTAL:  {:0>2}", app.items.len()),
            Style::default().fg(CYAN_DIM),
        )),
        Line::from(""),
    ];
    // This part creates the "moving" data look
    for i in 0..15 {
        let noise = format!("{:x}", (now / 50) ^ (i * 0xABC));
        telemetry.push(Line::from(Span::styled(
            format!("0x{}", &noise[..8.min(noise.len())]),
            Style::default().fg(DIM),
        )));
    }
    f.render_widget(
        Paragraph::new(telemetry).block(
            Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(CYAN_DARK)),
        ),
        workspace[2],
    );

    // --- 3. LOGS ---
    if let Some(log) = app.logs.last() {
        f.render_widget(
            Paragraph::new(format!(" 📡 SYS_LOG: {}", log))
                .style(Style::default().fg(DIM).add_modifier(Modifier::ITALIC)),
            chunks[2],
        );
    }

    // --- 4. FOOTER ---
    let footer_text = Line::from(vec![
        Span::styled(" ENTER ", Style::default().fg(BG).bg(CYAN)),
        Span::raw(" OPEN  "),
        Span::styled(" SPACE ", Style::default().fg(BG).bg(CYAN_DIM)),
        Span::raw(" MARK  "),
        Span::styled(" D ", Style::default().fg(BG).bg(GREEN)),
        Span::raw(" EXTRACT  "),
        Span::styled(" ESC ", Style::default().fg(BG).bg(BLUE)),
        Span::raw(" BACK "),
    ]);
    f.render_widget(
        Paragraph::new(footer_text)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(DIM)),
            )
            .alignment(Alignment::Center),
        chunks[3],
    );
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

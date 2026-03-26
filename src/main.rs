mod app;
mod github_api;
mod ui;

use crate::{
    app::{App, CurrentScreen},
    github_api::{fetch_contents, parse_github_url},
};
use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;

#[tokio::main]
async fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen);
    stdout().execute(SetCursorStyle::BlinkingUnderScore);
    let _ = enable_raw_mode();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new("Yukari-dev".to_string(), "Netmon".to_string());

    loop {
        terminal.draw(|f| {
            ui::render(f, &app);
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.screen {
                        CurrentScreen::FileList => match key.code {
                            KeyCode::Char('j') | KeyCode::Down => app.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.previous(),
                            KeyCode::Esc => app.screen = CurrentScreen::Input,
                            _ => {}
                        },
                        CurrentScreen::Input => {
                            match key.code {
                                KeyCode::Enter => {
                                    if let Ok((owner, repo)) = parse_github_url(&app.input_buffer) {
                                        app.owner = owner;
                                        app.repo = repo;
                                        app.loading = true;
                                        app.screen = app::CurrentScreen::Loading;

                                        match fetch_contents(&app.owner, &app.repo, "").await {
                                            Ok(fetched_items) => {
                                                app.items = fetched_items;
                                                app.loading = false;
                                                app.input_buffer.clear();
                                                app.cursor_position = 0;
                                                app.screen = app::CurrentScreen::FileList;
                                            }
                                            Err(_) => {
                                                app.loading = false;
                                                app.screen = app::CurrentScreen::Input;
                                            }
                                        }
                                    }
                                }
                                KeyCode::Char(c) => {
                                    app.enter_char(c);
                                }
                                KeyCode::Backspace => {
                                    app.delete_char();
                                }
                                KeyCode::Left => {
                                    app.move_cursor_left();
                                }
                                KeyCode::Right => {
                                    app.move_cursor_right();
                                }
                                KeyCode::Esc => {
                                    break; // Exit app
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    stdout().execute(SetCursorStyle::DefaultUserShape);
    let _ = disable_raw_mode();
    Ok(())
}

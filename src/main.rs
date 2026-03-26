mod app;
mod download;
mod github_api;
mod ui;

use crate::{
    app::{App, CurrentScreen},
    download::download_recursive,
    github_api::{fetch_contents, parse_github_url},
};
use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::SetCursorStyle,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io::stdout;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = stdout().execute(EnterAlternateScreen);
    let _ = stdout().execute(SetCursorStyle::BlinkingUnderScore);
    let _ = enable_raw_mode();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new("Yukari-dev".to_string(), "Netmon".to_string());

    let (tx, mut rx) = mpsc::channel::<String>(100);

    loop {
        while let Ok(msg) = rx.try_recv() {
            app.add_log(msg);
        }
        terminal.draw(|f| {
            ui::render(f, &app);
        })?;

        if event::poll(std::time::Duration::from_millis(16))?
            && let event::Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match app.screen {
                CurrentScreen::FileList => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char(' ') => {
                        if let CurrentScreen::FileList = app.screen {
                            app.toggle_mark();
                        }
                    }
                    KeyCode::Char('a') => {
                        app.toggle_select_all();
                    }
                    KeyCode::Char('d') => {
                        let owner = app.owner.clone();
                        let repo = app.repo.clone();
                        let marked_items: Vec<_> = app
                            .items
                            .iter()
                            .filter(|i| app.marked_paths.contains(&i.path))
                            .cloned()
                            .collect();
                        let tx_clone = tx.clone();
                        tokio::spawn(async move {
                            for item in marked_items {
                                let name = item.name.clone();
                                if download_recursive(&owner, &repo, item).await.is_ok() {
                                    let _ =
                                        tx_clone.send(format!("SUCCESS: Extracted {}", name)).await;
                                }
                            }
                        });
                    }
                    KeyCode::Esc => {
                        if app.current_path.to_string_lossy() == "" {
                            app.screen = CurrentScreen::Input;
                        } else if let CurrentScreen::FileList = app.screen {
                            app.go_back();
                            let path_str = app.current_path.to_string_lossy().to_string();
                            if let Ok(new_items) =
                                fetch_contents(&app.owner, &app.repo, &path_str).await
                            {
                                app.items = new_items;
                                app.list_state.select(Some(0));
                                app.screen = CurrentScreen::FileList;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        app.enter_directory();
                        let path_str = app.current_path.to_string_lossy().to_string();
                        if let Ok(new_items) =
                            fetch_contents(&app.owner, &app.repo, &path_str).await
                        {
                            app.items = new_items;
                            app.list_state.select(Some(0));
                            app.screen = CurrentScreen::FileList;
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Input => match key.code {
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
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = stdout().execute(SetCursorStyle::DefaultUserShape);
    let _ = disable_raw_mode();
    Ok(())
}

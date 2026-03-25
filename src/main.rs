use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{layout::*, prelude::*, widgets::*};
use std::io::{Result, stdout};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    let _ = enable_raw_mode();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
    Ok(())
}

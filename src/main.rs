use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut running = true;

    // Game Loop
    while running {
        // 1. Handle Input (just quitting for now)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => running = false,
                    _ => {}
                }
            }
        }

        // 2. Update

        // 3. Draw
        // Clear screen
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(
            stdout,
            cursor::MoveTo(10, 10),
            SetForegroundColor(Color::Cyan),
            Print("λ")
        )?;
    }
    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

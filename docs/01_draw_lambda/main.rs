// commit 01 — something on screen
// Goal: see a λ character on the alternate screen.
// This is the entire foundation. Everything else builds on these ~15 lines.

use crossterm::{
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    // Raw mode: keypresses arrive immediately, no echo, no line buffering.
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen, // clean slate, restores on exit
        cursor::Hide,                   // no blinking cursor over our character
    )?;

    // Draw λ at column 10, row 10
    execute!(
        stdout(),
        cursor::MoveTo(10, 10),
        SetForegroundColor(Color::Cyan),
        Print("λ"),
    )?;

    // Wait 2 seconds so you can see it, then clean up
    std::thread::sleep(std::time::Duration::from_secs(2));

    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

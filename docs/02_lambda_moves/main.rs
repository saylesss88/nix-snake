// commit 02 — the λ moves
// Goal: a game loop. The λ crawls right across the screen.
// We introduce: position state, a loop, clearing between frames.

use crossterm::{
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout(); // one handle, reused every frame
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut x: u16 = 0;
    let mut y: u16 = 10;

    // The game loop. Runs until we manually break out.
    // Each iteration = one frame.
    for _ in 0..40 {
        // Clear last frame. Without this, the old λ stays on screen
        // and you get a trail of characters.
        execute!(out, terminal::Clear(terminal::ClearType::All))?;

        execute!(
            out,
            cursor::MoveTo(x, y),
            SetForegroundColor(Color::Cyan),
            Print("λ"),
        )?;

        // flush() sends all buffered output to the terminal right now.
        // Without this, output might sit in an internal buffer and never appear.
        out.flush()?;

        x += 1; // move right one column each frame
        std::thread::sleep(Duration::from_millis(80));
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

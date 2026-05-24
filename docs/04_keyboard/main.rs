// commit 04 — keyboard steering + clean exit
// Goal: arrow keys change direction, q quits cleanly.
// We introduce: event::poll, non-blocking input, match on KeyCode.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};
use std::time::Duration;

type Direction = (i16, i16);
const RIGHT: Direction = (1, 0);
const LEFT:  Direction = (-1, 0);
const UP:    Direction = (0, -1); // y increases downward, so "up" = -1
const DOWN:  Direction = (0, 1);

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (width, height) = terminal::size()?;
    let mut x: u16 = width / 2;
    let mut y: u16 = height / 2;
    let mut dir: Direction = RIGHT;
    let mut running = true;

    while running {
        // Non-blocking input check.
        // poll(0ms) = "is there a keypress waiting RIGHT NOW?"
        // Returns immediately. If we used event::read() directly it would
        // block here forever waiting for a key — game loop stops dead.
        if event::poll(Duration::from_millis(0))?
            // Pattern match: only proceed if it's a Key event.
            // The `..` ignores all other fields of KeyEvent we don't need.
            && let Event::Key(KeyEvent { code, .. }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => running = false,
                KeyCode::Left  => dir = LEFT,
                KeyCode::Right => dir = RIGHT,
                KeyCode::Up    => dir = UP,
                KeyCode::Down  => dir = DOWN,
                _ => {} // ignore everything else
            }
        }

        let (w, h) = terminal::size()?; // re-query in case of resize
        let next_x = (i32::from(x) + i32::from(dir.0)).rem_euclid(i32::from(w));
        let next_y = (i32::from(y) + i32::from(dir.1)).rem_euclid(i32::from(h));
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        { x = next_x as u16; y = next_y as u16; }

        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print("λ"))?;
        out.flush()?;

        std::thread::sleep(Duration::from_millis(80));
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

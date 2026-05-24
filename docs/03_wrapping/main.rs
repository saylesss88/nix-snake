// commit 03 — wrapping + direction
// Goal: the λ wraps at screen edges and has a direction concept.
// We introduce: terminal::size(), rem_euclid, type Direction.

use crossterm::{
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};
use std::time::Duration;

// Direction is a 2D delta: how much x and y change each frame.
// i16 because deltas can be -1 (left, up).
type Direction = (i16, i16);

const RIGHT: Direction = (1, 0);

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (width, height) = terminal::size()?;
    let mut x: u16 = 0;
    let mut y: u16 = height / 2;
    let dir: Direction = RIGHT;

    loop {
        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print("λ"))?;
        out.flush()?;

        // Move one step in `dir`.
        //
        // WHY not just `x += dir.0`?
        // x is u16, dir.0 is i16 — different types, can't add directly.
        // Also, u16 can't go negative, so wrapping would panic in debug mode.
        //
        // The safe path: convert everything to i32 (fits both), do the math,
        // use rem_euclid to wrap into [0, width), cast back to u16.
        //
        // rem_euclid vs %:
        //   -1 % 40  = -1   (remainder, sign follows dividend)
        //   -1_i32.rem_euclid(40) = 39  (always in [0, n))
        // When the snake exits the left edge you want 39, not -1.
        let next_x = (i32::from(x) + i32::from(dir.0)).rem_euclid(i32::from(width));
        let next_y = (i32::from(y) + i32::from(dir.1)).rem_euclid(i32::from(height));

        // Safe: rem_euclid guarantees [0, width) which fits in u16
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        {
            x = next_x as u16;
            y = next_y as u16;
        }

        std::thread::sleep(Duration::from_millis(80));
    }
}

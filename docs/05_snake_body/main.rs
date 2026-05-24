// commit 05 — Snake struct with a body
// Goal: the λ has a tail. Introduce VecDeque and the Snake struct.
// The single (x,y) position becomes a deque of segments.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;

type Direction = (i16, i16);
const RIGHT: Direction = (1, 0);
const LEFT:  Direction = (-1, 0);
const UP:    Direction = (0, -1);
const DOWN:  Direction = (0, 1);

struct Snake {
    // body[0] = head. body[last] = tail tip.
    // VecDeque because snake movement = push_front (new head) + pop_back (drop tail).
    // Vec::insert(0, ...) is O(n) — shifts every element. VecDeque::push_front is O(1).
    body: VecDeque<(u16, u16)>,
    dir: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10)); // head
        body.push_back((9, 10));  // body
        body.push_back((8, 10));  // tail
        Self { body, dir: RIGHT }
    }

    fn update(&mut self, width: u16, height: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        let nx = (i32::from(hx) + i32::from(self.dir.0)).rem_euclid(i32::from(width));
        let ny = (i32::from(hy) + i32::from(self.dir.1)).rem_euclid(i32::from(height));
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        self.body.push_front((nx as u16, ny as u16));
        // Pop the tail — this is what makes the snake "move" without growing.
        // When we eat food we'll skip this pop, which grows the snake by 1.
        self.body.pop_back();
    }

    const fn set_direction(&mut self, new_dir: Direction) {
        // Block 180° reversal: opposite directions sum to (0,0).
        // RIGHT + LEFT = (1,0)+(-1,0) = (0,0) → blocked
        // RIGHT + DOWN = (1,0)+(0,1) = (1,1) → allowed
        if (self.dir.0 + new_dir.0 != 0) || (self.dir.1 + new_dir.1 != 0) {
            self.dir = new_dir;
        }
    }

    fn check_self_collision(&self) -> bool {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        // Does any segment after the head share the head's position?
        self.body.iter().skip(1).any(|&(x, y)| x == hx && y == hy)
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut snake = Snake::new();
    let mut running = true;

    while running {
        if event::poll(Duration::from_millis(0))?
            && let Event::Key(KeyEvent { code, .. }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => running = false,
                KeyCode::Left  => snake.set_direction(LEFT),
                KeyCode::Right => snake.set_direction(RIGHT),
                KeyCode::Up    => snake.set_direction(UP),
                KeyCode::Down  => snake.set_direction(DOWN),
                _ => {}
            }
        }

        let (w, h) = terminal::size()?;
        snake.update(w, h);

        if snake.check_self_collision() {
            // Flash red, then reset — simple death feedback
            execute!(out,
                crossterm::style::SetBackgroundColor(Color::Red),
                terminal::Clear(terminal::ClearType::All)
            )?;
            out.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(out,
                crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
                terminal::Clear(terminal::ClearType::All)
            )?;
            snake = Snake::new();
        }

        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        for (i, &(x, y)) in snake.body.iter().enumerate() {
            let ch = if i == 0 { "λ" } else { "o" };
            execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print(ch))?;
        }
        out.flush()?;

        std::thread::sleep(Duration::from_millis(100));
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

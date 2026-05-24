// commit 06 — food and eating
// Goal: food appears, eating it grows the snake.
// We introduce: Food struct, rand, the grow trick.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor, SetBackgroundColor},
    terminal,
};
use rand::RngExt;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;

type Direction = (i16, i16);
const RIGHT: Direction = (1, 0);
const LEFT:  Direction = (-1, 0);
const UP:    Direction = (0, -1);
const DOWN:  Direction = (0, 1);

struct Food {
    x: u16,
    y: u16,
    symbol: char,
}

impl Food {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(0..width),
            // saturating_sub(1): reserve bottom row for status bar.
            // Also guards against height==0 underflowing to 65535.
            y: rng.random_range(0..height.saturating_sub(1)),
            symbol: if rng.random_bool(0.5) { '❄' } else { '📦' },
        }
    }

    fn respawn(&mut self, width: u16, height: u16) {
        let mut rng = rand::rng();
        self.x = rng.random_range(0..width);
        self.y = rng.random_range(0..height.saturating_sub(1));
        self.symbol = if rng.random_bool(0.5) { '❄' } else { '📦' };
    }
}

struct Snake {
    body: VecDeque<(u16, u16)>,
    dir: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10));
        body.push_back((9, 10));
        body.push_back((8, 10));
        Self { body, dir: RIGHT }
    }

    fn reset(&mut self) {
        self.body.clear();
        self.body.push_back((10, 10));
        self.body.push_back((9, 10));
        self.body.push_back((8, 10));
        self.dir = RIGHT;
    }

    fn update(&mut self, width: u16, height: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        let nx = (i32::from(hx) + i32::from(self.dir.0)).rem_euclid(i32::from(width));
        let ny = (i32::from(hy) + i32::from(self.dir.1)).rem_euclid(i32::from(height));
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        self.body.push_front((nx as u16, ny as u16));
        self.body.pop_back();
    }

    const fn set_direction(&mut self, new_dir: Direction) {
        if (self.dir.0 + new_dir.0 != 0) || (self.dir.1 + new_dir.1 != 0) {
            self.dir = new_dir;
        }
    }

    fn check_self_collision(&self) -> bool {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        self.body.iter().skip(1).any(|&(x, y)| x == hx && y == hy)
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (w, h) = terminal::size()?;
    let mut snake = Snake::new();
    let mut food = Food::new(w, h);
    let mut score: u32 = 0;
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
        snake.update(w, h - 1); // h-1: keep snake out of status bar row

        if snake.check_self_collision() {
            execute!(out, SetBackgroundColor(Color::Red), terminal::Clear(terminal::ClearType::All))?;
            out.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(out, SetBackgroundColor(Color::Reset), terminal::Clear(terminal::ClearType::All))?;
            snake.reset();
            score = 0;
        }

        // Did the head land on food?
        // We check AFTER update() so we're comparing the new head position.
        if let Some(&(hx, hy)) = snake.body.front()
            && hx == food.x
            && hy == food.y
        {
            food.respawn(w, h);
            // Grow: push a duplicate of the current tail.
            // On the next update(), the real tail pops and this duplicate
            // becomes the new tail — net result: snake is 1 longer.
            if let Some(&tail) = snake.body.back() {
                snake.body.push_back(tail);
            }
            score += 10;
        }

        // Draw
        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        execute!(out, cursor::MoveTo(food.x, food.y), SetForegroundColor(Color::Red), Print(food.symbol))?;
        for (i, &(x, y)) in snake.body.iter().enumerate() {
            let ch = if i == 0 { "λ" } else { "o" };
            execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print(ch))?;
        }
        execute!(out, cursor::MoveTo(0, h - 1), SetForegroundColor(Color::Yellow), Print(format!("Score: {score}")))?;
        out.flush()?;

        std::thread::sleep(Duration::from_millis(100));
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

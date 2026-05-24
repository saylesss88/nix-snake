// commit 07 — speed scaling
// Goal: snake speeds up as score increases, floor at 25ms.
// The game is now fully playable with increasing difficulty.

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

struct Food { x: u16, y: u16, symbol: char }
impl Food {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::rng();
        Self { x: rng.random_range(0..w), y: rng.random_range(0..h.saturating_sub(1)),
               symbol: if rng.random_bool(0.5) { '❄' } else { '📦' } }
    }
    fn respawn(&mut self, w: u16, h: u16) {
        let mut rng = rand::rng();
        self.x = rng.random_range(0..w);
        self.y = rng.random_range(0..h.saturating_sub(1));
        self.symbol = if rng.random_bool(0.5) { '❄' } else { '📦' };
    }
}

struct Snake { body: VecDeque<(u16, u16)>, dir: Direction }
impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10)); body.push_back((9, 10)); body.push_back((8, 10));
        Self { body, dir: RIGHT }
    }
    fn reset(&mut self) {
        self.body.clear();
        self.body.push_back((10, 10)); self.body.push_back((9, 10)); self.body.push_back((8, 10));
        self.dir = RIGHT;
    }
    fn update(&mut self, w: u16, h: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        let nx = (i32::from(hx)+i32::from(self.dir.0)).rem_euclid(i32::from(w));
        let ny = (i32::from(hy)+i32::from(self.dir.1)).rem_euclid(i32::from(h));
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        self.body.push_front((nx as u16, ny as u16));
        self.body.pop_back();
    }
    const fn set_direction(&mut self, d: Direction) {
        if (self.dir.0+d.0 != 0) || (self.dir.1+d.1 != 0) { self.dir = d; }
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
    let mut speed = Duration::from_millis(100);
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
        snake.update(w, h - 1);

        if snake.check_self_collision() {
            execute!(out, SetBackgroundColor(Color::Red), terminal::Clear(terminal::ClearType::All))?;
            out.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(out, SetBackgroundColor(Color::Reset), terminal::Clear(terminal::ClearType::All))?;
            snake.reset();
            score = 0;
            speed = Duration::from_millis(100); // reset speed on death
        }

        if let Some(&(hx, hy)) = snake.body.front()
            && hx == food.x && hy == food.y
        {
            food.respawn(w, h);
            if let Some(&tail) = snake.body.back() { snake.body.push_back(tail); }
            score += 10;

            // Speed up by 5% per food. Floor at 25ms.
            //
            // WHY millis() / 20 == 5%?  Because 1/20 = 0.05 = 5%.
            //
            // WHY u64::try_from()?
            // as_millis() returns u128 (supports very long durations).
            // from_millis() takes u64.
            // try_from() returns Err if the value doesn't fit in u64.
            // expect() turns that into a panic with a clear message.
            // In practice speed is tiny so this never fails — the expect
            // documents that assumption explicitly rather than hiding it.
            //
            // WHY saturating_sub + .max()?
            // Two guards in one chain:
            //   saturating_sub: if reduction > speed, clamp to 0 not underflow
            //   .max(25ms): never go below 25ms regardless
            let reduction = u64::try_from(speed.as_millis() / 20)
                .expect("speed reduction fits in u64");
            speed = speed
                .saturating_sub(Duration::from_millis(reduction))
                .max(Duration::from_millis(25));
        }

        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        execute!(out, cursor::MoveTo(food.x, food.y), SetForegroundColor(Color::Red), Print(food.symbol))?;
        for (i, &(x, y)) in snake.body.iter().enumerate() {
            let ch = if i == 0 { "λ" } else { "o" };
            execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print(ch))?;
        }
        let status = format!("Score: {} | Speed: {}ms | q to quit", score, speed.as_millis());
        execute!(out, cursor::MoveTo(0, h-1), SetForegroundColor(Color::Yellow), Print(status))?;
        out.flush()?;

        std::thread::sleep(speed);
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

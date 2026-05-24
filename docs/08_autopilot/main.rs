// commit 08 — autopilot mode
// Goal: press 'a' for the snake to steer itself toward food.
// We introduce: Mode enum, derive(PartialEq), greedy pathfinding.

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

// WHY an enum and not a bool?
// `bool` works: `let auto = true`. But reading `if auto { ... }` later
// you'd wonder "auto what?" An enum is self-documenting. It also scales —
// adding a third mode means adding a variant, not converting bools to ints.
#[derive(PartialEq)] // needed so we can write `mode == Mode::Auto`
enum Mode { Auto, Manual }

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

    fn autopilot(&mut self, food_x: u16, food_y: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        // Greedy: move toward food along whichever axis is misaligned.
        // Prioritizes X first (arbitrary). Will eventually trap itself
        // once the snake is long — that's the interesting limitation to observe.
        if hx < food_x && self.dir != LEFT       { self.set_direction(RIGHT); }
        else if hx > food_x && self.dir != RIGHT { self.set_direction(LEFT); }
        else if hy < food_y && self.dir != UP    { self.set_direction(DOWN); }
        else if hy > food_y && self.dir != DOWN  { self.set_direction(UP); }
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (w, h) = terminal::size()?;
    let mut snake = Snake::new();
    let mut food = Food::new(w, h);
    let mut mode = Mode::Auto; // start in auto so it immediately does something visible
    let mut score: u32 = 0;
    let mut speed = Duration::from_millis(100);
    let mut running = true;

    while running {
        if event::poll(Duration::from_millis(0))?
            && let Event::Key(KeyEvent { code, .. }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => running = false,
                KeyCode::Char('a') => mode = Mode::Auto,
                // Any arrow key: switch to manual AND steer
                KeyCode::Left  => { mode = Mode::Manual; snake.set_direction(LEFT); }
                KeyCode::Right => { mode = Mode::Manual; snake.set_direction(RIGHT); }
                KeyCode::Up    => { mode = Mode::Manual; snake.set_direction(UP); }
                KeyCode::Down  => { mode = Mode::Manual; snake.set_direction(DOWN); }
                _ => {}
            }
        }

        if mode == Mode::Auto {
            snake.autopilot(food.x, food.y);
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
            speed = Duration::from_millis(100);
        }

        if let Some(&(hx, hy)) = snake.body.front()
            && hx == food.x && hy == food.y
        {
            food.respawn(w, h);
            if let Some(&tail) = snake.body.back() { snake.body.push_back(tail); }
            score += 10;
            let reduction = u64::try_from(speed.as_millis() / 20).expect("fits in u64");
            speed = speed.saturating_sub(Duration::from_millis(reduction)).max(Duration::from_millis(25));
        }

        execute!(out, terminal::Clear(terminal::ClearType::All))?;
        execute!(out, cursor::MoveTo(food.x, food.y), SetForegroundColor(Color::Red), Print(food.symbol))?;
        for (i, &(x, y)) in snake.body.iter().enumerate() {
            let ch = if i == 0 { "λ" } else { "o" };
            execute!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print(ch))?;
        }
        let mode_label = if mode == Mode::Auto { "AUTO (arrows to play)" } else { "MANUAL (a for auto)" };
        let status = format!("{} | Score: {} | Speed: {}ms", mode_label, score, speed.as_millis());
        execute!(out, cursor::MoveTo(0, h-1), SetForegroundColor(Color::Yellow), Print(status))?;
        out.flush()?;

        std::thread::sleep(speed);
    }

    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

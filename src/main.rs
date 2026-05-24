// commit 09 — refactor: extract functions + panic hook
// Goal: main() reads like a script. Add panic safety.
// No new features — pure reorganization + one important bug fix.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use rand::RngExt;
use std::collections::VecDeque;
use std::io::{Write, stdout};
use std::time::Duration;

type Direction = (i16, i16);
pub const RIGHT: Direction = (1, 0);
pub const LEFT:  Direction = (-1, 0);
pub const UP:    Direction = (0, -1);
pub const DOWN:  Direction = (0, 1);

#[derive(PartialEq)]
enum Mode { Auto, Manual }

struct Food { x: u16, y: u16, symbol: char }
impl Food {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(0..width),
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
    fn update(&mut self, max_width: u16, max_height: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        let nx = (i32::from(hx)+i32::from(self.dir.0)).rem_euclid(i32::from(max_width));
        let ny = (i32::from(hy)+i32::from(self.dir.1)).rem_euclid(i32::from(max_height));
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        self.body.push_front((nx as u16, ny as u16));
        self.body.pop_back();
    }
    const fn set_direction(&mut self, new_dir: Direction) {
        if (self.dir.0+new_dir.0 != 0) || (self.dir.1+new_dir.1 != 0) { self.dir = new_dir; }
    }
    fn check_self_collision(&self) -> bool {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        self.body.iter().skip(1).any(|&(x, y)| x == hx && y == hy)
    }
    fn autopilot(&mut self, food_x: u16, food_y: u16) {
        let (hx, hy) = *self.body.front().expect("snake has no body");
        if hx < food_x && self.dir != LEFT       { self.set_direction(RIGHT); }
        else if hx > food_x && self.dir != RIGHT { self.set_direction(LEFT); }
        else if hy < food_y && self.dir != UP    { self.set_direction(DOWN); }
        else if hy > food_y && self.dir != DOWN  { self.set_direction(UP); }
    }
}

fn setup_terminal() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
    Ok(())
}

fn restore_terminal() -> std::io::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn handle_input(mode: &mut Mode, snake: &mut Snake, running: &mut bool) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(0))?
        && let Event::Key(KeyEvent { code, .. }) = event::read()?
    {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => *running = false,
            KeyCode::Char('a') => *mode = Mode::Auto,
            KeyCode::Left  => { *mode = Mode::Manual; snake.set_direction(LEFT);  }
            KeyCode::Right => { *mode = Mode::Manual; snake.set_direction(RIGHT); }
            KeyCode::Up    => { *mode = Mode::Manual; snake.set_direction(UP);    }
            KeyCode::Down  => { *mode = Mode::Manual; snake.set_direction(DOWN);  }
            _ => {}
        }
    }
    Ok(())
}

fn handle_collision(snake: &mut Snake, out: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(out, SetBackgroundColor(Color::Red), terminal::Clear(terminal::ClearType::All))?;
    out.flush()?;
    std::thread::sleep(Duration::from_millis(200));
    execute!(out, SetBackgroundColor(Color::Reset), terminal::Clear(terminal::ClearType::All))?;
    snake.reset();
    Ok(())
}

fn draw_game(
    out: &mut std::io::Stdout,
    snake: &Snake,
    food: &Food,
    mode: &Mode,
    score: u32,
    speed: Duration,
    term_rows: u16,
) -> std::io::Result<()> {
    queue!(out, terminal::Clear(terminal::ClearType::All))?;
    queue!(out, cursor::MoveTo(food.x, food.y), SetForegroundColor(Color::Red), Print(food.symbol))?;
    for (i, &(x, y)) in snake.body.iter().enumerate() {
        let ch = if i == 0 { "λ" } else { "o" };
        queue!(out, cursor::MoveTo(x, y), SetForegroundColor(Color::Cyan), Print(ch))?;
    }
    let mode_label = if *mode == Mode::Auto { "AUTO (arrows to play)" } else { "MANUAL (a for auto)" };
    let status = format!("{} | Score: {} | Speed: {}ms", mode_label, score, speed.as_millis());
    queue!(out, cursor::MoveTo(0, term_rows-1), SetForegroundColor(Color::Yellow), Print(status))?;
    out.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    setup_terminal()?;

    // Panic hook: restore terminal even if something panics.
    // Without this, a panic in raw mode leaves the shell broken.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        original_hook(info);
    }));

    let mut out = stdout();
    let mut snake = Snake::new();
    let mut running = true;
    let (w, h) = terminal::size()?;
    let mut food = Food::new(w, h);
    let mut mode = Mode::Auto;
    let mut score: u32 = 0;
    let mut speed = Duration::from_millis(100);

    while running {
        handle_input(&mut mode, &mut snake, &mut running)?;

        if mode == Mode::Auto {
            snake.autopilot(food.x, food.y);
        }

        let (term_cols, term_rows) = terminal::size()?;
        snake.update(term_cols, term_rows - 1);

        if snake.check_self_collision() {
            handle_collision(&mut snake, &mut out)?;
            score = 0;
            speed = Duration::from_millis(100);
        }

        if let Some(&(hx, hy)) = snake.body.front()
            && hx == food.x && hy == food.y
        {
            let (w, h) = terminal::size()?;
            food.respawn(w, h);
            if let Some(&tail) = snake.body.back() { snake.body.push_back(tail); }
            score += 10;
            let reduction = u64::try_from(speed.as_millis() / 20).expect("fits in u64");
            speed = speed.saturating_sub(Duration::from_millis(reduction)).max(Duration::from_millis(25));
        }

        draw_game(&mut out, &snake, &food, &mode, score, speed, term_rows)?;
        std::thread::sleep(speed);
    }

    restore_terminal()?;
    Ok(())
}

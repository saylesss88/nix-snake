use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self},
};
use rand::RngExt;
use std::collections::VecDeque;
use std::io::{Write, stdout};
use std::time::Duration;

#[derive(PartialEq)]
enum Mode {
    Auto,
    Manual,
}

struct Food {
    x: u16,
    y: u16,
    symbol: char, // '❄' or '📦'
}

impl Food {
    // We need to know screen size to spawn randomly
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(0..width),
            y: rng.random_range(0..height),
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

// direction constants
const RIGHT: (i16, i16) = (1, 0);
const LEFT: (i16, i16) = (-1, 0);
const UP: (i16, i16) = (0, -1);
const DOWN: (i16, i16) = (0, 1);

struct Snake {
    // body holds all segments. body[0] is the head.
    body: VecDeque<(u16, u16)>,
    dir: (i16, i16),
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10)); // Head
        body.push_back((9, 10)); // Tail segment 1
        body.push_back((8, 10)); // Tail segment 2 (start with length 3)
        Self {
            body,
            dir: RIGHT, // Start moving right
        }
    }

    fn check_self_collision(&self) -> bool {
        let (head_x, head_y) = *self.body.front().expect("Snake has no body");
        // Skip the first element (the head) and check the rest
        self.body
            .iter()
            .skip(1)
            .any(|&(x, y)| x == head_x && y == head_y)
    }

    // Helper to reset snake on death
    fn reset(&mut self) {
        self.body.clear();
        self.body.push_back((10, 10));
        self.body.push_back((9, 10));
        self.body.push_back((8, 10));
        self.dir = RIGHT;
    }

    fn update(&mut self, max_width: u16, max_height: u16) {
        let (head_x, head_y) = *self.body.front().expect("Snake has no body");

        // Use i32::from() for lossless conversion from u16/i16 to i32
        let next_x = (i32::from(head_x) + i32::from(self.dir.0)).rem_euclid(i32::from(max_width));

        let next_y = (i32::from(head_y) + i32::from(self.dir.1)).rem_euclid(i32::from(max_height));

        // We still use 'as u16' at the very end to put it back in the deque

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        self.body.push_front((next_x as u16, next_y as u16));
        self.body.pop_back();
    }

    const fn set_direction(&mut self, new_dir: (i16, i16)) {
        // Prevent 180 turns (banning reversing)
        // If current is RIGHT (1,0) and new is LEFT (-1,0), sum is (0,0).
        // This simple check works for opposite cardinal directions.
        if (self.dir.0 + new_dir.0 != 0) || (self.dir.1 + new_dir.1 != 0) {
            self.dir = new_dir;
        }
    }

    // The AI Logic
    fn autopilot(&mut self, food_x: u16, food_y: u16) {
        let (head_x, head_y) = *self.body.front().unwrap();

        // Determine ideal direction
        // Prioritize X movement first (arbitrary choice)
        if head_x < food_x && self.dir != LEFT {
            self.set_direction(RIGHT);
        } else if head_x > food_x && self.dir != RIGHT {
            self.set_direction(LEFT);
        } else if head_y < food_y && self.dir != UP {
            self.set_direction(DOWN);
        } else if head_y > food_y && self.dir != DOWN {
            self.set_direction(UP);
        }
        // Else: keep going current direction (or pick random safe turn if stuck)
    }
}
fn setup_terminal() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    Ok(())
}

fn restore_terminal() -> std::io::Result<()> {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn handle_input(mode: &mut Mode, snake: &mut Snake, running: &mut bool) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(KeyEvent { code, .. }) = event::read()?
    {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => *running = false,
            KeyCode::Char('a') => *mode = Mode::Auto,
            KeyCode::Left => {
                *mode = Mode::Manual;
                snake.set_direction(LEFT);
            }
            KeyCode::Right => {
                *mode = Mode::Manual;
                snake.set_direction(RIGHT);
            }
            KeyCode::Up => {
                *mode = Mode::Manual;
                snake.set_direction(UP);
            }
            KeyCode::Down => {
                *mode = Mode::Manual;
                snake.set_direction(DOWN);
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_collision(snake: &mut Snake, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(
        stdout,
        SetBackgroundColor(Color::Red),
        terminal::Clear(terminal::ClearType::All)
    )?;
    stdout.flush()?;
    std::thread::sleep(Duration::from_millis(200));
    execute!(
        stdout,
        SetBackgroundColor(Color::Reset),
        terminal::Clear(terminal::ClearType::All)
    )?;
    snake.reset();
    Ok(())
}

fn draw_game(
    stdout: &mut std::io::Stdout,
    snake: &Snake,
    food: &Food,
    mode: &Mode,
    score: u32,
    speed: Duration,
    term_rows: u16,
) -> std::io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mode_text = match mode {
        Mode::Auto => "AUTO (Press Arrows to Play)",
        Mode::Manual => "MANUAL (Press 'a' for Auto)",
    };
    execute!(
        stdout,
        cursor::MoveTo(0, term_rows - 1),
        SetForegroundColor(Color::Yellow),
        Print(mode_text)
    )?;

    execute!(
        stdout,
        cursor::MoveTo(food.x, food.y),
        SetForegroundColor(Color::Red),
        Print(food.symbol)
    )?;

    for (i, point) in snake.body.iter().enumerate() {
        let symbol = if i == 0 { "λ" } else { "o" };
        execute!(
            stdout,
            cursor::MoveTo(point.0, point.1),
            SetForegroundColor(Color::Cyan),
            Print(symbol)
        )?;
    }

    let status_text = match mode {
        Mode::Auto => format!("AUTO | Score: {} | Speed: {}ms", score, speed.as_millis()),
        Mode::Manual => format!("MANUAL | Score: {} | Speed: {}ms", score, speed.as_millis()),
    };
    execute!(
        stdout,
        cursor::MoveTo(0, term_rows - 1),
        SetForegroundColor(Color::Yellow),
        Print(status_text)
    )?;
    stdout.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    setup_terminal()?;
    let mut stdout = stdout();
    let mut snake = Snake::new();
    let mut running = true;
    let (w, h) = terminal::size()?;
    let mut food = Food::new(w, h);
    let mut mode = Mode::Auto;
    let mut score = 0;
    let mut speed = Duration::from_millis(100);

    while running {
        handle_input(&mut mode, &mut snake, &mut running)?;
        if mode == Mode::Auto {
            snake.autopilot(food.x, food.y);
        }
        let (term_cols, term_rows) = terminal::size()?;
        snake.update(term_cols, term_rows - 1);

        if snake.check_self_collision() {
            handle_collision(&mut snake, &mut stdout)?;
            score = 0;
            speed = Duration::from_millis(100);
        }

        if let (Some(&(head_x, head_y)), Some(&tail)) = (snake.body.front(), snake.body.back())
            && head_x == food.x
            && head_y == food.y
        {
            let (w, h) = terminal::size()?;
            food.respawn(w, h);
            snake.body.push_back(tail);
            score += 10;
            speed = speed
                .saturating_sub(Duration::from_millis(10))
                .max(Duration::from_millis(40));
        }

        draw_game(&mut stdout, &snake, &food, &mode, score, speed, term_rows)?;
    }
    restore_terminal()?;
    Ok(())
}

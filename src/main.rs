use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;

struct Food {
    x: u16,
    y: u16,
    symbol: char, // '❄' or '📦'
}

impl Food {
    // We need to know screen size to spawn randomly
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            // Randomly pick a symbol
            symbol: if rng.gen_bool(0.5) { '❄' } else { '📦' },
        }
    }

    fn respawn(&mut self, width: u16, height: u16) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0..width);
        self.y = rng.gen_range(0..height);
        self.symbol = if rng.gen_bool(0.5) { '❄' } else { '📦' };
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

    fn update(&mut self, max_width: u16, max_height: u16) {
        // 1. Get current head
        let (head_x, head_y) = *self.body.front().expect("Snake has no body");

        // 2. Calculate new head position
        // We cast to i16 to handle negative checking, then wrap/clamp
        let next_x = head_x as i16 + self.dir.0;
        let next_y = head_y as i16 + self.dir.1;

        // 3. Boundary Check (Simple Wrap-around for screensaver vibe)
        // If it goes off screen (approx 80x24 for now, we'll get real size later), wrap it.
        // Let's use a fixed size for this step to prevent crashes.
        let width = max_width as i16;
        let height = max_height as i16;

        // Force the variable type to be i16
        let new_x: i16 = if next_x < 0 {
            width - 1
        } else if next_x >= width {
            0
        } else {
            next_x
        };

        let new_y: i16 = if next_y < 0 {
            height - 1
        } else if next_y >= height {
            0
        } else {
            next_y
        };

        // Now we explicitly cast to u16 only at the very end
        self.body.push_front((new_x as u16, new_y as u16));

        // 5. Remove tail (Simulate movement, not growing yet)
        self.body.pop_back();
    }
}

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut snake = Snake::new();
    let mut running = true;

    let (w, h) = terminal::size()?;
    let mut food = Food::new(w, h);

    // Game Loop
    while running {
        // 1. Handle Input (just quitting for now)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => running = false,
                    // Manual Controls
                    KeyCode::Left => snake.dir = LEFT,
                    KeyCode::Right => snake.dir = RIGHT,
                    KeyCode::Up => snake.dir = UP,
                    KeyCode::Down => snake.dir = DOWN,
                    _ => {}
                }
            }
        }

        // 2. Update
        let (term_cols, term_rows) = terminal::size()?;
        snake.update(term_cols, term_rows);

        // 3. Draw
        // Clear screen
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        execute!(
            stdout,
            cursor::MoveTo(food.x, food.y),
            SetForegroundColor(Color::Red), // Make food Red or White
            Print(food.symbol)
        )?;

        if let Some(&(head_x, head_y)) = snake.body.front() {
            if head_x == food.x && head_y == food.y {
                // Respawn food
                let (w, h) = terminal::size()?;
                food.respawn(w, h);

                // GROW: To grow, we just skip the `pop_back` we did in update().
                // But since `update` does pop_back automatically, the easiest way
                // is to just add a dummy tail segment back, OR change `update` logic.
                if let Some(&tail) = snake.body.back() {
                    snake.body.push_back(tail);
                }
            }
        }

        for (i, point) in snake.body.iter().enumerate() {
            // Head gets the Lambda, body gets a different char
            let symbol = if i == 0 { "λ" } else { "o" };
            execute!(
                stdout,
                cursor::MoveTo(point.0, point.1),
                SetForegroundColor(Color::Cyan),
                Print(symbol)
            )?;

            // Flush output
            stdout.flush()?;
        }
    }
    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

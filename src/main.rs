use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use std::time::Duration;

// direction constants
const RIGHT: (i16, i16) = (1, 0);
const LEFT: (i16, i16) = (-1, 0);
const UP: (i16, i16) = (0, -1);
const DOWN: (i16, i16) = (0, 1);

struct Snake {
    // Position (using u16 because terminal coordinates are unsigned)
    // BUT we might use i16 for math to avoid underflow checks initially
    // Let's stick to u16 for pos and cast when moving.
    x: u16,
    y: u16,
    // Direction (dx, dy)
    dir: (i16, i16),
}

impl Snake {
    fn new() -> Self {
        Self {
            x: 10,
            y: 10,
            dir: RIGHT, // Start moving right
        }
    }

    fn update(&mut self) {
        // Simple move: cast to i16, add dir, cast back to u16
        // (We will add boundary checks next step, for now it might crash if it hits 0)
        self.x = (self.x as i16 + self.dir.0) as u16;
        self.y = (self.y as i16 + self.dir.1) as u16;
    }
}

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut snake = Snake::new();
    let mut running = true;

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
        snake.update();

        // 3. Draw
        // Clear screen
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(
            stdout,
            cursor::MoveTo(snake.x, snake.y),
            SetForegroundColor(Color::Cyan),
            Print("λ")
        )?;

        // Flush output
        stdout.flush()?;
    }
    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

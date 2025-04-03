use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::*,
    widgets::*,
    Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

mod game;
use game::*;

fn main() -> io::Result<()> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Game state
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    // Main game loop
    loop {
        terminal.draw(|f| ui(f, &game))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up if game.direction != SnakeDirection::Down => {
                        game.direction = SnakeDirection::Up
                    }
                    KeyCode::Down if game.direction != SnakeDirection::Up => {
                        game.direction = SnakeDirection::Down
                    }
                    KeyCode::Left if game.direction != SnakeDirection::Right => {
                        game.direction = SnakeDirection::Left
                    }
                    KeyCode::Right if game.direction != SnakeDirection::Left => {
                        game.direction = SnakeDirection::Right
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            //Height: terminal height -3 score widget, -2 borders
            game.update(terminal.size()?.width-2, terminal.size()?.height-5);
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, game: &Game) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    // Score widget
    let score = Paragraph::new(format!("Score: {}", game.score))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(score, chunks[0]);

    // Game area
    let game_area = chunks[1];
    
    //Borders
    let area_height = game_area.height - 2;
    let area_width = game_area.width - 2;
    let mut buffer = vec![vec![' '; area_width as usize]; area_height as usize];


    // Draw snake
    for &(x, y) in &game.snake {
        if x < area_width && y < area_height {
            buffer[y as usize][x as usize] = '█';
        }
    }

    // Draw food
    let (food_x, food_y) = game.food;
    if food_x < area_width && food_y < area_height {
        buffer[food_y as usize][food_x as usize] = '●';
    }

    // Create text widget for the game area
    let text: Vec<Line> = buffer
        .iter()
        .map(|row| Line::from(row.iter().collect::<String>()))
        .collect();
    let game_text = Paragraph::new(text)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(game_text, game_area);

    // Game over message
    if game.game_over {
        let game_over = Paragraph::new("Game Over! Press 'q' to quit")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(Clear, game_area);
        f.render_widget(game_over, game_area);
    }
}

use std::{
    collections::VecDeque,
};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub snake: VecDeque<(u16, u16)>,
    pub food: (u16, u16),
    pub direction: SnakeDirection,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((10, 10));
        // snake.push_back((10, 11));
        // snake.push_back((10, 12));
        // snake.push_back((10, 13));
        // snake.push_back((10, 14));
        // snake.push_back((10, 15));
        Self {
            snake,
            food: (5, 5),
            direction: SnakeDirection::Right,
            score: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self, width: u16, height: u16) {
        if self.game_over {
            return;
        }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            SnakeDirection::Up => (head.0, head.1.saturating_sub(1)),
            SnakeDirection::Down => (head.0, head.1.saturating_add(1)),
            SnakeDirection::Left => (head.0.saturating_sub(1), head.1),
            SnakeDirection::Right => (head.0.saturating_add(1), head.1),
        };
        
        // Check for collisions with walls
        if new_head.0 >= width || new_head.1 >= height {
            self.game_over = true;
            return;
        }

        // Check for collisions with self
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        // Check if food is eaten
        if new_head == self.food {
            self.score += 1;
            self.generate_food(width, height);
        } else {
            self.snake.pop_back();
        }
    }

    fn generate_food(&mut self, width: u16, height: u16) {
        let mut rng = rand::thread_rng();
        loop {
            let food = (
                rng.gen_range(0..width),
                rng.gen_range(0..height),
            );
            if !self.snake.contains(&food) {
                self.food = food;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.snake.len(), 1);
        assert_eq!(game.snake.front().unwrap(), &(10, 10));
        assert_eq!(game.food, (5, 5));
        assert_eq!(game.score, 0);
        assert_eq!(game.game_over, false);
    }
    
    #[test]
    fn test_update_movement() {
        let mut game = Game::new();
        game.update(20, 20);
        assert_eq!(game.snake.front().unwrap(), &(11, 10)); // Moved right
        
        game.direction = SnakeDirection::Down;
        game.update(20, 20);
        assert_eq!(game.snake.front().unwrap(), &(11, 11)); // Moved down
    }
    
    #[test]
    fn test_collision_with_wall() {
        let mut game = Game::new();
        game.snake.clear();
        game.snake.push_back((8, 9));
        game.direction = SnakeDirection::Right;
        game.update(10, 10); // Move right to edge
        assert_eq!(game.game_over, false);
        
        game.update(10, 10); // Try to move beyond edge
        assert_eq!(game.game_over, true);
    }
    
    #[test]
    fn test_eating_food() {
        let mut game = Game::new();
        game.snake.clear();
        game.snake.push_back((4, 5));
        game.direction = SnakeDirection::Right;
        game.food = (5, 5);
        
        let initial_length = game.snake.len();
        game.update(20, 20);
        assert_eq!(game.snake.len(), initial_length + 1);
        assert_eq!(game.score, 1);
        assert_ne!(game.food, (5, 5)); // Food should be regenerated
    }
}
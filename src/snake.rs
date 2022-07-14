use std::collections::VecDeque;

use crate::random::get_random_number;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>,
    direction: Direction,
    next_direction: Direction,
    pub food: Position,
    pub finished: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        SnakeGame {
            width,
            height,
            snake: VecDeque::from([Position {
                x: 2.min(width - 1),
                y: height / 2,
            }]),
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position {
                x: (width - 2).max(0),
                y: height / 2,
            },
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Down, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Left)
            | (Direction::Left, Direction::Right) => {}
            (_, direction) => self.next_direction = direction,
        }
    }

    fn is_valid(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    pub fn game_loop(&mut self) {
        if self.finished {
            return;
        }

        self.direction = self.next_direction;

        let head = &self.snake[0];

        let new_head = match &self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Position {
                x: head.x - 1,
                y: head.y,
            },
        };

        if !self.is_valid(&new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| Position { x, y }))
                    .filter(|pos| !self.snake.contains(&pos))
                    .collect::<Vec<_>>();

                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }

                self.food = free_positions[get_random_number(0, free_positions.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::snake::*;

    #[test]
    fn it_works() {
        let game = SnakeGame::new(20, 20);

        println!("{:?}", game);
    }

    #[test]
    fn it_changes_direction_correctly() {
        let mut game = SnakeGame::new(20, 20);
        assert!(matches!(game.direction, Direction::Right));

        game.change_direction(Direction::Left);
        game.game_loop();
        assert!(matches!(game.direction, Direction::Right));

        game.change_direction(Direction::Right);
        game.game_loop();
        assert!(matches!(game.direction, Direction::Right));

        game.change_direction(Direction::Up);
        game.game_loop();
        assert!(matches!(game.direction, Direction::Up));

        game.change_direction(Direction::Down);
        game.game_loop();
        assert!(matches!(game.direction, Direction::Up));

        game.change_direction(Direction::Left);
        game.game_loop();
        assert!(matches!(game.direction, Direction::Left));
    }

    #[test]
    fn eating_food_grows_the_snake() {
        let mut game = SnakeGame::new(10, 10);
        assert!(game.snake.len() == 1);

        let head = game.snake[0];
        game.food = Position {
            x: head.x + 2,
            y: head.y,
        }; // the food is 2 cells in front

        game.game_loop();
        assert!(game.snake.len() == 1);

        game.game_loop();
        assert!(game.snake.len() == 2);
    }
    #[test]
    fn running_into_wall_ends_game() {
        let mut game = SnakeGame::new(2, 2);
        assert!(game.finished == false);

        game.game_loop();

        assert!(game.finished);
    }
}

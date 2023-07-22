use std::vec;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    // 导入js函数
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    // option是特殊的枚举
    next_cell: Option<SnakeCell>,
    // 豆子的位置
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    // 积分
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        World {
            width,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            size,
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        // 如果头的下一个位置是第二个cell，说明直接往回
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        // self.next_cell = Option::Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // pub fn oopsie(&mut self) {
    //     self.snake.body = vec![SnakeCell(2028)];
    // }
    // wasm不能导出rs引用(&)、vec

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing!"),
            None => String::from("No Status"),
        }
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }

                // let next_cell = self.gen_next_snake_cell(&self.snake.direction);
                // self.snake.body[0] = next_cell;

                let len = self.snake.body.len();

                // 每个cell都移动到前一个cell的位置
                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                // 吃到豆子
                if self.reward_cell == Some(self.snake_head_idx()) {
                    if (self.snake_length() < self.size) {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;

        loop {
            // 保证不生成到蛇身上
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        // 当前在第几行
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                // 向右的话，row是去掉余数得到的，所以要row+1
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    // 回到该行第一格
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = (row) * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}
// wasm-pack build --target web

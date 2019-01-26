use std::io::prelude::*;
use std::time::{Duration, Instant};

use andrew::Endian;
use rand::prelude::*;

use yuxa::{
    ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent,
    YuxaWindow,
};

const TILES: usize = 15;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Static,
}

struct Snake {
    pos: Vec<(usize, usize)>,
    apple_pos: (usize, usize),
    dir: Direction,
    score: usize,
    grow: bool,
    last_move: Instant,
    game_over: bool,
    font_data: Vec<u8>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut font_data = Vec::new();
        std::fs::File::open(
            andrew::text::fontconfig::FontConfig::new()
                .unwrap()
                .get_regular_family_fonts("sans")
                .unwrap()
                .iter()
                .filter(|p| p.extension().unwrap() == "ttf")
                .nth(0)
                .unwrap(),
        )
        .unwrap()
        .read_to_end(&mut font_data)
        .unwrap();
        let middle = (TILES / 2 - 1, TILES / 2 - 1);
        let mut snake = Snake {
            pos: vec![middle, (middle.0 - 1, middle.1), (middle.0 - 2, middle.1)],
            apple_pos: (0, 0),
            dir: Direction::Static,
            score: 0,
            grow: false,
            last_move: Instant::now(),
            game_over: false,
            font_data,
        };
        snake.spawn_apple();
        snake
    }

    pub fn spawn_apple(&mut self) {
        let mut rand_gen = thread_rng();
        self.apple_pos = (rand_gen.gen_range(1, TILES), rand_gen.gen_range(1, TILES));
        while self.pos.contains(&self.apple_pos) {
            self.apple_pos = (rand_gen.gen_range(1, TILES), rand_gen.gen_range(1, TILES));
        }
    }

    pub fn move_snake(&mut self) {
        let grow_pos = *self.pos.last().unwrap();

        for i in (1..self.pos.len()).rev() {
            self.pos[i] = self.pos[i - 1];
        }

        if self.grow {
            self.pos.push(grow_pos);
            self.grow = false;
        }

        match self.dir {
            Direction::Up => {
                if self.pos[0].1 == 0 {
                    self.game_over()
                };
                self.pos[0].1 -= 1
            }
            Direction::Down => {
                if self.pos[0].1 == TILES - 1 {
                    self.game_over()
                };
                self.pos[0].1 += 1
            }
            Direction::Left => {
                if self.pos[0].0 == 0 {
                    self.game_over()
                };
                self.pos[0].0 -= 1
            }
            Direction::Right => {
                if self.pos[0].0 == TILES - 1 {
                    self.game_over()
                };
                self.pos[0].0 += 1
            }
            Direction::Static => {}
        };

        if self.pos[1..].contains(&self.pos[0]) {
            self.game_over();
        }
    }

    pub fn draw(&mut self, window: &mut YuxaWindow) {
        let dimensions: (u32, u32) = window.get_inner_size().unwrap().to_physical(1.).into();
        let dimensions = (dimensions.0 as usize, dimensions.1 as usize);
        let mut buffer = vec![0; dimensions.0 * dimensions.1 * 4];
        let mut canvas = andrew::Canvas::new(
            &mut buffer,
            dimensions.0,
            dimensions.1,
            dimensions.0 * 4,
            Endian::Big,
        );
        let scale = (
            dimensions.0 as f32 / TILES as f32,
            dimensions.1 as f32 / TILES as f32,
        );

        // Draw background
        let background = andrew::shapes::rectangle::Rectangle::new(
            (0, 0),
            dimensions,
            None,
            Some([255, 0, 0, 0]),
        );
        canvas.draw(&background);

        // Draw apple
        let apple = andrew::shapes::rectangle::Rectangle::new(
            (
                (self.apple_pos.0 as f32 * scale.0) as usize,
                (self.apple_pos.1 as f32 * scale.1) as usize,
            ),
            (scale.0 as usize, scale.1 as usize),
            None,
            Some([255, 255, 0, 0]),
        );
        canvas.draw(&apple);

        // Draw snake
        for pos in &self.pos {
            let body = andrew::shapes::rectangle::Rectangle::new(
                (
                    (pos.0 as f32 * scale.0) as usize,
                    (pos.1 as f32 * scale.1) as usize,
                ),
                (scale.0 as usize, scale.1 as usize),
                None,
                Some([255, 0, 255, 0]),
            );
            canvas.draw(&body);
        }

        // Draw score
        let text = andrew::text::Text::new(
            (dimensions.0 / 20, dimensions.1 / 20),
            [255, 0, 0, 255],
            &self.font_data,
            dimensions.0 as f32 / 30.,
            1.0,
            self.score.to_string(),
        );
        canvas.draw(&text);

        // Draw game over
        if self.game_over {
            let mut text = andrew::text::Text::new(
                (400, 50),
                [255, 0, 0, 255],
                &self.font_data,
                dimensions.0 as f32 / 20.,
                1.0,
                "Game Over".to_string(),
            );
            text.pos = (dimensions.0 / 2 - text.get_width() / 2, dimensions.0 / 10);
            canvas.draw(&text);
        }

        window.draw_argb8888_bytes(&canvas.buffer);
    }

    pub fn update(&mut self, window: &mut YuxaWindow) {
        if self.pos[0] == self.apple_pos {
            self.score += 1;
            self.grow = true;
            self.spawn_apple();
            println!("Score: {}", self.score);
        }

        if self.dir != Direction::Static && self.last_move.elapsed().subsec_millis() > 150 {
            // Move snake and redraw the screen
            self.move_snake();
            self.draw(window);
            self.last_move = Instant::now();
        }
    }

    pub fn game_over(&mut self) {
        println!("Game Over");
        self.game_over = true;
    }
}

fn main() {
    // Window setup
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title("Yuxa Window");
    let mut window = YuxaWindow::new(window, &events_loop).unwrap();

    // Game setup
    let mut snake = Snake::new();

    loop {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Refresh => snake.draw(&mut window),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match virtual_code {
                        // Get user input
                        VirtualKeyCode::W => {
                            if snake.pos[1].1 != snake.pos[0].1 - 1 {
                                snake.dir = Direction::Up
                            }
                        }
                        VirtualKeyCode::S => {
                            if snake.pos[1].1 != snake.pos[0].1 + 1 {
                                snake.dir = Direction::Down
                            }
                        }
                        VirtualKeyCode::A => {
                            if snake.pos[1].0 != snake.pos[0].0 - 1 {
                                snake.dir = Direction::Left
                            }
                        }
                        VirtualKeyCode::D => {
                            if snake.pos[1].0 != snake.pos[0].0 + 1 {
                                snake.dir = Direction::Right
                            }
                        }
                        VirtualKeyCode::Space => snake.dir = Direction::Static,
                        _ => {}
                    },

                    _ => {}
                }
            }
        });

        // Update game if not over
        if !snake.game_over {
            snake.update(&mut window);
        }

        // 60 fps limit
        std::thread::sleep(Duration::from_millis(16));
    }
}

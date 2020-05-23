use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, GameResult};
use rand;
use rand::{Rng, ThreadRng};
use std::time::{Duration, Instant};

mod snake;
use snake::{Snake, Direction};

mod food; use food::Food;

fn main() -> GameResult {
    // Make a Context.
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "Gray Olson")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Game::new(ctx)?;

    // Run!
    match event::run(ctx, events_loop, &mut my_game) {
        Ok(_) => println!("12314123141 Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }

    Ok(())
}

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
const UPDATES_PER_SECOND: f32 = 17.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    x: i16,
    y: i16,
}

/// This is a trait that provides a modulus function that works for negative values
/// rather than just the standard remainder op (%) which does not. We'll use this
/// to get our snake to wrap from one side of the game board around to the other
/// when it goes off the top, bottom, left, or right side of the screen.
trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}
impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        // Because of our trait bounds, we can now apply these operators.
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}

impl Position {
    fn random(rng: &mut ThreadRng, max_x: i16, max_y: i16) -> Position {
        let x: i16 = rng.gen_range(0, max_x);
        let y: i16 = rng.gen_range(0, max_y);

        Position { x, y }
    }
}

impl From<Position> for graphics::Rect {
    fn from(pos: Position) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

struct Assets {
    font: graphics::Font,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let font = graphics::Font::new(ctx, "/arial.ttf")?;

        Ok(Self { font })
    }
}

struct Game {
    snake: Snake,
    food: Vec<Food>,
    last_update: Instant,
    ms_per_update: Duration,
    score: u32,

    assets: Assets,

    rng: ThreadRng,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();

        let snake_start = Position::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        let assets = Assets::new(ctx)?;

        Ok(Game {
            snake: Snake::new(snake_start),
            food: Vec::new(),
            last_update: Instant::now(),
            ms_per_update: Duration::from_millis(MILLIS_PER_UPDATE),
            score: 0,

            assets,

            rng,
        })
    }

    fn restart(&mut self) {
        self.score = 0;

        let r = Position::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
        self.snake = Snake::new(r);
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= self.ms_per_update && self.snake.is_alive {
            let mut to_delete: Vec<usize> = Vec::new();

            for (i, f) in self.food.iter_mut().enumerate() {
                if self.snake.check_collison(&f.r) {
                    dbg!("FOOOD colision");
                    self.snake.grow();
                    self.score += f.v;

                    to_delete.push(i);
                }
            }

            to_delete.iter().for_each(|i| {
                self.food.remove(*i);
            });

            self.snake.update();

            if self.food.len() == 0 {
                let r = Position::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
                let f = Food::at_random_position(r, &mut self.rng);
                self.food.push(f);
            }

            self.last_update = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.snake.draw(ctx)?;

        for f in &self.food {
            f.draw(ctx)?;
        }

        let score_str = format!("Score: {}", self.score);
        let score_display = graphics::Text::new((score_str, self.assets.font, 32.0));
        graphics::draw(
            ctx,
            &score_display,
            (
                ggez::nalgebra::Point2::new(10.0, 10.0),
                0.0,
                graphics::WHITE,
            ),
        )?;

        if !self.snake.is_alive {
            let game_over_str = format!("GAME OVER! Press R to restart.");
            let game_over_display =
                graphics::Text::new((game_over_str.clone(), self.assets.font, 32.0));

            let x = (SCREEN_SIZE.0 / 2.0) - (game_over_str.len() as f32 * 6.0);
            let y = (SCREEN_SIZE.1 / 2.0) - 32.0;

            graphics::draw(
                ctx,
                &game_over_display,
                (
                    ggez::nalgebra::Point2::new(x, y),
                    0.0,
                    [1.0, 0.0, 0.0, 1.0].into(),
                ),
            )?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        dbg!(keycode);
        match keycode {
            event::KeyCode::Left => self.snake.set_direction(Direction::Left),
            event::KeyCode::Right => self.snake.set_direction(Direction::Right),
            event::KeyCode::Down => self.snake.set_direction(Direction::Down),
            event::KeyCode::Up => self.snake.set_direction(Direction::Up),
            event::KeyCode::R => {
                self.restart();
            }
            _ => {}
        }
    }
}
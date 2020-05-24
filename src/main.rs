use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};
use rand::{Rng, ThreadRng};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

mod snake;
use snake::{Direction, Snake};

mod food;
use food::Food;

mod power_up;
use power_up::{PowerType, PowerUp};

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

pub struct Game {
    snake: Snake,
    food: Vec<Food>,
    last_update: Instant,
    ms_per_update: Duration,

    score: u16,
    scored: bool,

    assets: Assets,

    rng: ThreadRng,

    active_power_ups: Vec<(Rc<dyn PowerUp>, Instant)>,
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
            scored: false,

            assets,

            rng,

            active_power_ups: Vec::new(),
        })
    }

    fn restart(&mut self) {
        self.score = 0;

        let r = Position::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
        self.snake = Snake::new(r);
    }

    fn create_new_food(&mut self) {
        let r = Position::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
        let p = Food::random_power(&mut self.rng);

        let f = Food::at_random_position(r, p, &mut self.rng);
        self.food.push(f);
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= self.ms_per_update && self.snake.is_alive {
            let mut to_delete: Vec<usize> = Vec::new();

            let food = self.food.clone();
            for (i, f) in food.iter().enumerate() {
                if self.snake.check_collison(&f.r) {
                    if let Some(power_up) = &f.power {
                        let power_up = power_up.clone();

                        power_up.on_activation(self);

                        self.active_power_ups.push((power_up, Instant::now()));
                    }

                    self.snake.grow();
                    self.score += 1;
                    self.scored = true;

                    to_delete.push(i);
                }
            }

            to_delete.iter().for_each(|i| {
                self.food.remove(*i);
            });
            to_delete.clear();

            self.snake.update();

            let power_ups = self.active_power_ups.clone();
            for (i, (power_up, added_at)) in power_ups.iter().enumerate() {
                power_up.on_update(self);

                if power_up.should_remove(*added_at) {
                    to_delete.push(i);
                }
            }
            to_delete.iter().for_each(|i| {
                self.active_power_ups.remove(*i);
            });

            if self.food.len() == 0 {
                self.create_new_food();
            }

            self.scored = false;
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

        let mut p_string = String::new();
        for (power_up, _) in self.active_power_ups.clone().iter() {
            p_string.push_str(&format!("{}\n", power_up));
        }

        let p_display = graphics::Text::new((p_string, self.assets.font, 32.0));
        let x = SCREEN_SIZE.0 - (p_display.dimensions(ctx).0 + 10) as f32;
        graphics::draw(
            ctx,
            &p_display,
            (ggez::nalgebra::Point2::new(x, 10.0), 0.0, graphics::WHITE),
        )?;

        if !self.snake.is_alive {
            let game_over_str = format!("GAME OVER! Press R to restart.");
            let game_over_display =
                graphics::Text::new((game_over_str.clone(), self.assets.font, 32.0));

            let x = (SCREEN_SIZE.0 / 2.0) - (game_over_display.dimensions(ctx).0 / 2) as f32;
            let y = (SCREEN_SIZE.1 / 2.0) - (game_over_display.dimensions(ctx).1 / 2) as f32;

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

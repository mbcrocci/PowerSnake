use crate::{graphics, Context, GameResult, Position};
use rand::ThreadRng;

pub struct Food {
    pub r: Position,
    pub v: u32,
}

impl Food {
    pub fn at_random_position(start_pos: Position, _rng: &mut ThreadRng) -> Self {
        Self { r: start_pos, v: 1 }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.r.clone().into(),
            [1.0, 0.0, 1.0, 1.0].into(),
        )?;

        graphics::draw(ctx, &r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}

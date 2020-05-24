use crate::{graphics, Context, GameResult, Position, PowerType, PowerUp, Rc, Rng, ThreadRng};

use crate::power_up::{Invulnerability, ScoreMultiplier};

#[derive(Clone)]
pub struct Food {
    pub r: Position,
    pub power: Option<Rc<dyn PowerUp>>,
    power_type: PowerType,
}

impl Food {
    pub fn random_power(rng: &mut ThreadRng) -> PowerType {
        let probability: f32 = rng.gen_range(0.0, 1.0);

        if probability < 0.1 {
            PowerType::Invulnerability
        } else if probability < 0.25 {
            let v: u16 = rng.gen_range(0, 5);
            PowerType::ScoreMultiplier(v)
        } else {
            PowerType::None
        }
    }

    pub fn at_random_position(start_pos: Position, power: PowerType, _rng: &mut ThreadRng) -> Self {
        let power_up: Option<Rc<dyn PowerUp>> = match power {
            PowerType::None => None,
            PowerType::ScoreMultiplier(mult_by) => Some(Rc::new(ScoreMultiplier { mult_by })),
            PowerType::Invulnerability => Some(Rc::new(Invulnerability {})),
        };

        Self {
            r: start_pos,
            power: power_up,
            power_type: power,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = match self.power_type {
            PowerType::None => [1.0, 0.0, 1.0, 1.0].into(),
            PowerType::ScoreMultiplier(_) => [0.0, 1.0, 0.0, 1.0].into(),
            PowerType::Invulnerability => [1.0, 0.0, 0.0, 1.0].into(),
        };

        let r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.r.clone().into(),
            color,
        )?;

        graphics::draw(ctx, &r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}

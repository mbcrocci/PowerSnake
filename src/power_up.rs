use crate::{Duration, Game, Instant};

#[derive(Debug, Clone)]
pub enum PowerType {
    None,
    ScoreMultiplier(u16),
    Invulnerability,
}

pub trait PowerUp {
    fn on_activation(&self, _game: &mut Game) {}
    fn on_update(&self, _game: &mut Game) {}
    fn on_deactivation(&self, _game: &mut Game) {}
    fn should_remove(&self, _added_at: Instant) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct ScoreMultiplier {
    pub mult_by: u16,
}

impl PowerUp for ScoreMultiplier {
    fn on_activation(&self, _game: &mut Game) {}
    fn on_update(&self, game: &mut Game) {
        if game.scored {
            game.score -= 1;

            game.score += self.mult_by;
        }
    }
    fn on_deactivation(&self, _game: &mut Game) {}

    fn should_remove(&self, added_at: Instant) -> bool {
        Instant::now().duration_since(added_at) > Duration::from_secs(30)
    }
}

#[derive(Debug, Clone)]
pub struct Invulnerability {}

impl PowerUp for Invulnerability {
    fn on_activation(&self, _game: &mut Game) {}
    fn on_update(&self, game: &mut Game) {
        game.snake.is_alive = true;
    }
    fn on_deactivation(&self, _game: &mut Game) {}

    fn should_remove(&self, added_at: Instant) -> bool {
        Instant::now().duration_since(added_at) > Duration::from_secs(20)
    }
}

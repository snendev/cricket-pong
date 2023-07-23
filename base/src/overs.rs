use bevy_ecs::prelude::Resource;

use crate::Identity;

pub struct BowlScore {
    pub scorer: Identity,
    pub value: u16,
}

// AKA an "inning"
#[derive(Resource, Default)]
pub struct Over(Vec<BowlScore>);

pub enum BowlResult {
    None,
    ChangePositions,
    GameOver,
}

impl Over {
    pub fn get(&self, index: usize) -> Option<&BowlScore> {
        self.0.get(index)
    }

    pub fn push(&mut self, score: BowlScore) -> BowlResult {
        self.0.push(score);
        match self.0.len() {
            // switch sides after one over
            6 => BowlResult::ChangePositions,
            // end the game after two
            12 => BowlResult::GameOver,
            _ => BowlResult::None,
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

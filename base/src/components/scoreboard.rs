use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::{Property, Replicate, Serde};

use crate::components::player::Identity;

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub struct BowlScore {
    pub scorer: Identity,
    pub value: u16,
}

// AKA an "inning"
#[derive(Component, Replicate)]
pub struct Scoreboard {
    scores: Property<Vec<BowlScore>>,
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard::new_complete(Vec::new())
    }
}

impl Scoreboard {
    pub fn player_score(&self, identity: Identity) -> u16 {
        self.scores
            .iter()
            .filter_map(|score| {
                if score.scorer == identity {
                    Some(score.value)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn name() -> Name {
        Name::new("Scoreboard")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BowlResult {
    None,
    ChangePositions,
    GameOver,
}

impl Scoreboard {
    pub fn get(&self, index: usize) -> Option<&BowlScore> {
        self.scores.get(index)
    }

    pub fn push(&mut self, score: BowlScore) -> BowlResult {
        self.scores.push(score);
        match self.scores.len() {
            // switch sides after one over
            6 => BowlResult::ChangePositions,
            // end the game after two
            12 => BowlResult::GameOver,
            _ => BowlResult::None,
        }
    }

    pub fn clear(&mut self) {
        self.scores.clear();
    }
}

#[derive(Bundle)]
pub struct ScoreboardBundle {
    name: Name,
    scoreboard: Scoreboard,
}

impl Default for ScoreboardBundle {
    fn default() -> Self {
        Self {
            name: Scoreboard::name(),
            scoreboard: Scoreboard::default(),
        }
    }
}

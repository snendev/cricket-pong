use bevy_core::Name;
use bevy_ecs::prelude::{Bundle, Component};

use naia_bevy_shared::Serde;

use crate::components::player::Identity;

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub struct BowlScore {
    pub scorer: Identity,
    pub value: u8,
}

impl BowlScore {
    pub fn new(scorer: Identity, value: u8) -> Self {
        BowlScore { scorer, value }
    }
}

// AKA an "inning"
#[derive(Component, Default)]
pub struct Scoreboard {
    scores: Vec<BowlScore>,
}

impl Scoreboard {
    pub fn player_score(&self, identity: Identity) -> u8 {
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

    pub fn len(&self) -> usize {
        self.scores.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scores.is_empty()
    }

    pub fn force_set(&mut self, index: usize, score: BowlScore) {
        self.scores.truncate(index);
        self.scores.push(score);
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

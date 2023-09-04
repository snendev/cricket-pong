use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::{Entity, Event};

use cricket_pong_base::{
    actions::Action,
    components::{player::Identity, scoreboard::BowlScore},
};

pub struct PlayerAssignmentMessage {
    pub entity: Entity,
}

impl PlayerAssignmentMessage {
    pub fn new(entity: Entity) -> Self {
        PlayerAssignmentMessage { entity }
    }
}

impl Default for PlayerAssignmentMessage {
    fn default() -> Self {
        Self::new(Entity::PLACEHOLDER)
    }
}

#[derive(Debug, Event, Deserialize, Serialize)]
pub struct ActionMessage {
    pub entity: Entity,
    pub action: Option<Action>,
}

impl ActionMessage {
    pub fn new(entity: Entity, action: Option<Action>) -> Self {
        ActionMessage { entity, action }
    }
}

pub struct ScoreMessage {
    pub scorer: Identity,
    pub value: u8,
    pub index: usize,
}

impl ScoreMessage {
    pub fn new(score: BowlScore, index: usize) -> Self {
        ScoreMessage {
            scorer: score.scorer,
            value: score.value,
            index,
        }
    }
}

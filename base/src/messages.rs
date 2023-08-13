use crate::{
    actions::Action,
    components::{player::Identity, scoreboard::BowlScore},
};

pub struct PlayerAssignmentMessage {
    // pub entity: EntityProperty,
}

impl PlayerAssignmentMessage {
    pub fn new() -> Self {
        PlayerAssignmentMessage {
            // entity: EntityProperty::new(),
        }
    }
}

impl Default for PlayerAssignmentMessage {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ActionMessage {
    // pub entity: EntityProperty,
    pub action: Option<Action>,
}

impl ActionMessage {
    pub fn new(action: Option<Action>) -> Self {
        ActionMessage {
            // entity: EntityProperty::new(),
            action,
        }
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

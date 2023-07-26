use naia_bevy_shared::{EntityProperty, Message};

use crate::actions::Action;

#[derive(Message)]
pub struct PlayerAssignmentMessage {
    pub entity: EntityProperty,
}

impl PlayerAssignmentMessage {
    pub fn new() -> Self {
        PlayerAssignmentMessage {
            entity: EntityProperty::new(),
        }
    }
}

impl Default for PlayerAssignmentMessage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Message)]
pub struct ActionMessage {
    pub entity: EntityProperty,
    pub action: Option<Action>,
}

impl ActionMessage {
    pub fn new(action: Option<Action>) -> Self {
        ActionMessage {
            entity: EntityProperty::new(),
            action,
        }
    }
}

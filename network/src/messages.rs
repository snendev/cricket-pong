use bevy_replicon::prelude::{MapError, MapEventEntities};
use serde::{Deserialize, Serialize};

use bevy_ecs::{
    entity::EntityMap,
    prelude::{Entity, Event},
};

use cricket_pong_base::{actions::Action, components::player::Identity};

#[derive(Debug, Event, Deserialize, Serialize)]
pub struct PlayerAssignmentMessageEvent {
    pub identity: Identity,
}

impl PlayerAssignmentMessageEvent {
    pub fn new(identity: Identity) -> Self {
        PlayerAssignmentMessageEvent { identity }
    }
}

#[derive(Debug, Event, Deserialize, Serialize)]
pub struct ActionMessageEvent {
    pub entity: Entity,
    pub action: Option<Action>,
}

impl ActionMessageEvent {
    pub fn new(entity: Entity, action: Option<Action>) -> Self {
        ActionMessageEvent { entity, action }
    }
}

impl MapEventEntities for ActionMessageEvent {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapError> {
        if let Some(entity) = entity_map.get(self.entity) {
            self.entity = entity;
            Ok(())
        } else {
            Err(MapError(self.entity))
        }
    }
}

use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};

use naia_bevy_server::UserKey;

pub enum UserRoomParticipation {
    Undecided,
    Player,
    Spectator,
}

#[derive(Resource, Default)]
pub struct UserEntities {
    user_to_entity_map: HashMap<UserKey, Entity>,
    entity_to_user_map: HashMap<Entity, UserKey>,
}

impl UserEntities {
    pub fn get_entity(&self, user: &UserKey) -> Option<&Entity> {
        self.user_to_entity_map.get(user)
    }

    pub fn get_user(&self, entity: &Entity) -> Option<&UserKey> {
        self.entity_to_user_map.get(entity)
    }

    pub fn insert(&mut self, user_key: UserKey, entity: Entity) {
        self.user_to_entity_map.insert(user_key, entity);
        self.entity_to_user_map.insert(entity, user_key);
    }

    pub fn remove(&mut self, user: &UserKey) -> Option<Entity> {
        self.user_to_entity_map.remove(user).map(|entity| {
            self.entity_to_user_map.remove(&entity);
            entity
        })
    }
}

#[derive(Resource, Default)]
pub struct QueuedUsers {
    pub users: Vec<UserKey>,
}

use std::collections::{HashMap, HashSet};

use bevy_ecs::{entity::Entity, system::Resource};

use naia_bevy_server::{RoomKey, UserKey};

#[derive(Default)]
pub struct State<T: Default + std::fmt::Debug> {
    inner: T,
    scheduled: Option<T>,
    just_changed: Option<T>,
}

impl<T: Copy + Default + PartialEq + std::fmt::Debug> State<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: initial,
            scheduled: None,
            just_changed: None,
        }
    }

    pub fn inner(&self) -> T {
        self.inner
    }

    pub fn in_state(&self, value: T) -> bool {
        self.inner == value
    }

    pub fn just_changed_to(&self, value: T) -> bool {
        self.in_state(value) && self.just_changed.is_some()
    }

    pub fn set(&mut self, value: T) -> Option<T> {
        let replaced_value = self.scheduled.take();
        self.scheduled = Some(value);
        replaced_value
    }

    pub fn flush(&mut self) {
        if let Some(inner) = self.scheduled.take() {
            let prev_value = self.inner;
            self.inner = inner;
            self.just_changed = Some(prev_value);
        } else {
            self.just_changed = None;
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, PartialEq)]
pub enum LobbyState {
    #[default]
    FindingUsers,
    Setup,
    Active,
    Paused,
    Gameover,
}

#[derive(Default, Resource)]
pub struct LobbyStateMap(pub HashMap<RoomKey, State<LobbyState>>);

impl LobbyStateMap {
    pub fn insert(
        &mut self,
        room_key: RoomKey,
        lobby_state: LobbyState,
    ) -> Option<State<LobbyState>> {
        self.0.insert(room_key, State::new(lobby_state))
    }
}

pub enum UserRoomParticipation {
    Undecided,
    Player,
    Spectator,
}

#[derive(Default, Resource)]
pub struct ReadiedUsers(pub HashSet<UserKey>);

#[derive(Default, Resource)]
pub struct UserEntityMap {
    pub instance_to_room_map: HashMap<u64, RoomKey>,
    pub user_to_entity_map: HashMap<UserKey, Entity>,
    pub user_to_room_map: HashMap<UserKey, RoomKey>,
    pub entity_to_user_map: HashMap<Entity, UserKey>,
    pub entity_to_room_map: HashMap<Entity, RoomKey>,
}

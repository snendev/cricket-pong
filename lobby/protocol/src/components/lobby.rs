use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum LobbyState {
    Loading,
    Active,
    Unloading,
}

#[derive(Component, Replicate)]
pub struct GameLobby {
    pub state: Property<LobbyState>,
}

impl Default for GameLobby {
    fn default() -> Self {
        GameLobby::new_complete(LobbyState::Loading)
    }
}

impl GameLobby {
    pub fn is_loading(&self) -> bool {
        *self.state == LobbyState::Loading
    }

    pub fn is_active(&self) -> bool {
        *self.state == LobbyState::Active
    }

    pub fn is_unloading(&self) -> bool {
        *self.state == LobbyState::Unloading
    }

    pub fn activate(&mut self) {
        *self.state = LobbyState::Active;
    }

    pub fn unload(&mut self) {
        *self.state = LobbyState::Unloading;
    }
}

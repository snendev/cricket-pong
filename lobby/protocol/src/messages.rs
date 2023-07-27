use naia_bevy_shared::{Channel, Message, Serde};

#[derive(Clone, PartialEq, Serde)]
pub enum LobbyMessageData {
    Start,
    Pause,
}

#[derive(Message)]
pub struct LobbyMessage(LobbyMessageData);

impl LobbyMessage {
    pub fn start() -> Self {
        LobbyMessage(LobbyMessageData::Start)
    }

    pub fn pause() -> Self {
        LobbyMessage(LobbyMessageData::Pause)
    }

    pub fn inner(&self) -> &LobbyMessageData {
        &self.0
    }
}

#[derive(Channel)]
pub struct LobbyMessageChannel;

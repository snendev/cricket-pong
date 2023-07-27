use naia_bevy_shared::{ChannelDirection, ChannelMode, Protocol, ProtocolPlugin, ReliableSettings};

pub mod bundles;
pub mod components;
pub mod messages;

pub struct CommonLobbyProtocolPlugin;

impl ProtocolPlugin for CommonLobbyProtocolPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_channel::<messages::LobbyMessageChannel>(
                ChannelDirection::Bidirectional,
                ChannelMode::OrderedReliable(ReliableSettings::default()),
            )
            .add_message::<messages::LobbyMessage>();
    }
}

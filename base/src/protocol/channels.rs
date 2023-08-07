use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, Protocol, ReliableSettings, TickBufferSettings,
};

#[derive(Channel)]
pub struct PlayerActionsChannel;
impl PlayerActionsChannel {
    pub fn add_to_protocol(protocol: &mut Protocol) {
        protocol.add_channel::<PlayerActionsChannel>(
            ChannelDirection::ClientToServer,
            ChannelMode::TickBuffered(TickBufferSettings::default()),
        );
    }
}

#[derive(Channel)]
pub struct PlayerAssignmentChannel;

impl PlayerAssignmentChannel {
    pub fn add_to_protocol(protocol: &mut Protocol) {
        protocol.add_channel::<PlayerAssignmentChannel>(
            ChannelDirection::ServerToClient,
            ChannelMode::UnorderedReliable(ReliableSettings::default()),
        );
    }
}

#[derive(Channel)]
pub struct ScoreMessageChannel;

impl ScoreMessageChannel {
    pub fn add_to_protocol(protocol: &mut Protocol) {
        protocol.add_channel::<ScoreMessageChannel>(
            ChannelDirection::ServerToClient,
            ChannelMode::OrderedReliable(ReliableSettings::default()),
        );
    }
}

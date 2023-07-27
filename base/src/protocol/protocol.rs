use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol, ProtocolPlugin};

use common_lobby_protocol::CommonLobbyProtocolPlugin;

use crate::{
    components::{ball, batter, boundary, fielder, physics, wicket},
    protocol::{channels, messages},
};

struct CricketPongProtocolPlugin;

impl ProtocolPlugin for CricketPongProtocolPlugin {
    fn build(&self, protocol: &mut Protocol) {
        channels::PlayerActionsChannel::add_to_protocol(protocol);
        channels::PlayerAssignmentChannel::add_to_protocol(protocol);

        protocol
            .add_message::<messages::PlayerAssignmentMessage>()
            .add_message::<messages::ActionMessage>()
            .add_component::<ball::Ball>()
            .add_component::<batter::Batter>()
            .add_component::<wicket::Wicket>()
            .add_component::<fielder::Fielder>()
            .add_component::<boundary::Boundary>()
            .add_component::<physics::Transform>()
            .add_component::<physics::Velocity>()
            .add_component::<physics::ExternalImpulse>();
    }
}

pub fn protocol() -> Protocol {
    Protocol::builder()
        .tick_interval(Duration::from_millis(16))
        .link_condition(LinkConditionerConfig::good_condition())
        .add_plugin(CricketPongProtocolPlugin)
        .add_plugin(CommonLobbyProtocolPlugin)
        .build()
}

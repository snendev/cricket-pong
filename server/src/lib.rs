use std::fmt::Debug;

use bevy_app::{
    prelude::{App, Plugin, Startup, Update},
    First,
};
use bevy_ecs::{
    prelude::{Component, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
    system::Query,
};
use bevy_log::debug;
use bevy_utils::Duration;

use naia_bevy_server::{Plugin as NaiaServerPlugin, ReceiveEvents, ServerConfig};

use common_lobby_server::{resources::UserEntities, CommonLobbyPlugin, LobbySet};
use cricket_pong_game::{
    base::{
        components::{
            ball::Ball,
            batter::Batter,
            fielder::Fielder,
            phase::GamePhase,
            physics::{Rotation, Translation},
            wicket::Wicket,
        },
        protocol::protocol,
    },
    GameplayPlugin,
};

pub mod init;
pub mod matchmaking;
pub mod tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let mut server_config = ServerConfig {
            require_auth: false,
            ..Default::default()
        };
        server_config.ping.rtt_initial_estimate = Duration::from_millis(100);
        app.add_plugins(NaiaServerPlugin::new(server_config, protocol()))
            .add_plugins(CommonLobbyPlugin)
            .configure_sets(Update, (TickSet, LobbySet).in_set(ReceiveEvents))
            .init_resource::<UserEntities>()
            .add_systems(Startup, init::init)
            .add_systems(
                Update,
                (
                    matchmaking::pair_queued_users.after(LobbySet),
                    tick::send_score_mesasges.after(TickSet),
                    tick::update_entity_scopes.after(TickSet),
                ),
            )
            .add_plugins(GameplayPlugin::new(
                TickSet,
                tick::tick_events,
                common_lobby_server::subscribe_to_game_instances,
            ));
    }
}

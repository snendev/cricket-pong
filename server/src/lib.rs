use bevy_app::prelude::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::{IntoSystemConfigs, IntoSystemSetConfig, SystemSet};

use naia_bevy_server::{Plugin as NaiaServerPlugin, ReceiveEvents, ServerConfig};

use common_lobby_server::{resources::UserEntities, CommonLobbyPlugin, LobbySet};
use cricket_pong_game::{base::protocol::protocol, GameplayPlugin};

pub mod init;
pub mod matchmaking;
pub mod tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NaiaServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .add_plugins(CommonLobbyPlugin)
        .configure_set(Update, TickSet.in_set(ReceiveEvents))
        .init_resource::<UserEntities>()
        .add_systems(Startup, init::init)
        .add_systems(
            Update,
            matchmaking::pair_queued_users
                .after(LobbySet)
                .in_set(ReceiveEvents),
        )
        .add_systems(
            Update,
            tick::update_entity_scopes
                .after(TickSet)
                .in_set(ReceiveEvents),
        )
        .add_plugins(GameplayPlugin::new(
            TickSet,
            tick::tick_events,
            common_lobby_server::subscribe_to_game_instances,
        ));
    }
}

use std::fmt::Debug;

use bevy_app::{
    prelude::{App, Plugin, PluginGroup, Update},
    Startup,
};
use bevy_ecs::prelude::{
    system_adapter, Added, Commands, Component, Entity, IntoSystem, IntoSystemConfigs, Query,
    SystemSet,
};
use bevy_reflect::prelude::Reflect;

use bevy_replicon::{
    prelude::{
        ClientPlugin as RepliconClientPlugin, Replication, ReplicationPlugins,
        ServerPlugin as RepliconServerPlugin,
    },
    server::TickPolicy,
};

use cricket_pong_game::{
    base::{
        components::{
            ball::Ball,
            batter::Batter,
            boundary::Boundary,
            fielder::{Fielder, FielderTrack},
            instance::GameLobby,
            player::{PlayerOne, PlayerTwo},
            scoreboard::Scoreboard,
            transform::Transform,
            wicket::Wicket,
        },
        rapier::prelude::{ExternalImpulse, Velocity},
    },
    GameplayPlugin,
};
use network_base::{sync_to_replication, ReplicationStrategyPlugin};

mod init;
mod tick;
mod users;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ReplicationPlugins
                .build()
                .disable::<RepliconClientPlugin>()
                .set(RepliconServerPlugin {
                    tick_policy: TickPolicy::MaxTickRate(60),
                }),
        )
        .add_plugins(ReplicationStrategyPlugin)
        .add_systems(
            Startup,
            init::initialize_server.pipe(system_adapter::unwrap),
        )
        .init_resource::<users::QueuedUsers>()
        .add_systems(
            Update,
            (
                (
                    // all distinct entity "kinds" should have Replication component
                    attach_replication_to::<GameLobby>,
                    attach_replication_to::<PlayerOne>,
                    attach_replication_to::<PlayerTwo>,
                    attach_replication_to::<Ball>,
                    attach_replication_to::<Boundary>,
                    attach_replication_to::<Fielder>,
                    attach_replication_to::<FielderTrack>,
                    attach_replication_to::<Batter>,
                    attach_replication_to::<Wicket>,
                    attach_replication_to::<Scoreboard>,
                )
                    .before(TickSet),
                (
                    sync_to_replication::<Transform>,
                    sync_to_replication::<Velocity>,
                    sync_to_replication::<ExternalImpulse>,
                    sync_to_replication::<Fielder>,
                    sync_to_replication::<Batter>,
                    sync_to_replication::<Ball>,
                    sync_to_replication::<Scoreboard>,
                )
                    .after(TickSet),
                users::handle_user_connections,
                users::handle_room_cleanup,
                users::pair_queued_users,
                users::send_player_assignment_messages,
            ),
        )
        .add_plugins(GameplayPlugin::new(TickSet, tick::handle_actions));
    }
}

fn attach_replication_to<T: Component + Reflect>(
    mut commands: Commands,
    query: Query<Entity, Added<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Replication);
    }
}

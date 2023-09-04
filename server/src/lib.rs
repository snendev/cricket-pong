use std::fmt::Debug;

use bevy_app::{
    prelude::{App, Plugin, PluginGroup, Update},
    Startup,
};
use bevy_ecs::prelude::{
    system_adapter, Added, Commands, Component, Entity, In, IntoSystem, IntoSystemConfigs, Local,
    Query, Res, SystemSet,
};
use bevy_reflect::prelude::Reflect;

use bevy_replicon::prelude::{
    AppReplicationExt, ClientPlugin, Replication, ReplicationPlugins, SendPolicy, ServerEventAppExt,
};

use network_base::{messages::ActionMessage, sync_to_replication, Replicated};

use cricket_pong_game::{
    base::components::{
        batter::Batter,
        fielder::Fielder,
        instance::{GameLobby, PlayerID},
    },
    Actions, GameInstance, GameplayPlugin,
};

mod init;
// mod tick;
mod users;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

fn yield_local_ticks(actions: Res<Actions>, mut tick: Local<u16>) -> Vec<(u16, Actions)> {
    let result = (*tick, actions.clone());
    *tick += 1;
    vec![result]
}

fn noop(_: In<Vec<(GameInstance, Vec<Entity>)>>) {}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReplicationPlugins.build().disable::<ClientPlugin>())
            .replicate::<PlayerID>()
            .replicate::<GameLobby>()
            .replicate::<GameInstance>()
            .replicate::<Replicated<Fielder>>()
            .replicate::<Replicated<Batter>>()
            .add_server_event::<ActionMessage>(SendPolicy::Ordered)
            .add_systems(
                Startup,
                init::initialize_server.pipe(system_adapter::unwrap),
            )
            .init_resource::<users::UserEntities>()
            .init_resource::<users::QueuedUsers>()
            .add_systems(
                Update,
                (
                    attach_replication_to::<Fielder>,
                    attach_replication_to::<Batter>,
                    users::handle_user_connections,
                    users::handle_room_cleanup,
                    users::pair_queued_users,
                ),
            )
            .add_systems(Update, sync_to_replication::<Fielder>.after(TickSet))
            .add_plugins(GameplayPlugin::new(TickSet, yield_local_ticks, noop));
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

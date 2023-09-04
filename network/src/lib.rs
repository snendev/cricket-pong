use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{Component, Query, ReflectComponent};
use bevy_reflect::prelude::Reflect;

use bevy_replicon::prelude::{AppReplicationExt, SendPolicy, ServerEventAppExt};
use cricket_pong_base::components::{
    batter::Batter,
    fielder::Fielder,
    instance::{GameInstance, GameLobby, PlayerID},
};

pub mod messages;

#[derive(Clone, Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Replicated<T: Component + Default>(T);

pub fn sync_to_replication<T: Clone + Component + Default + Reflect>(
    mut query: Query<(&T, &mut Replicated<T>)>,
) {
    for (source, mut replicated) in query.iter_mut() {
        *replicated = Replicated(source.clone());
    }
}

pub struct ReplicationStrategyPlugin;

impl Plugin for ReplicationStrategyPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<PlayerID>()
            .replicate::<GameLobby>()
            .replicate::<GameInstance>()
            .replicate::<Replicated<Fielder>>()
            .replicate::<Replicated<Batter>>()
            .add_server_event::<messages::ActionMessage>(SendPolicy::Ordered);
    }
}

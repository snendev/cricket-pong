use bevy_app::prelude::{App, Plugin};
#[cfg(debug_assertions)]
use bevy_core::prelude::Name;
use bevy_ecs::prelude::{Commands, Component, Entity, Query, ReflectComponent};
use bevy_reflect::prelude::Reflect;

use bevy_replicon::prelude::{AppReplicationExt, ClientEventAppExt, SendPolicy, ServerEventAppExt};

use cricket_pong_base::{
    components::{
        ball::Ball,
        batter::Batter,
        boundary::Boundary,
        fielder::{Fielder, FielderPosition, FielderRing, FielderTrack},
        instance::{GameInstance, GameLobby, PlayerID},
        phase::GamePhase,
        player::{Identity, PlayerOne, PlayerTwo, Position},
        scoreboard::{BowlScore, Scoreboard},
        transform::Transform,
        wicket::Wicket,
    },
    rapier::prelude::{ExternalImpulse, Velocity},
};

pub mod messages;

// N.B. in order to call AppReplicationExt::replicate<Replicated<T>>,
// we must also register T and all property types of T
#[derive(Clone, Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Replicated<T: Component + Default>(T);

pub fn sync_to_replication<T: Clone + Component + Default + Reflect>(
    mut commands: Commands,
    mut query: Query<(Entity, &T, Option<&mut Replicated<T>>)>,
) {
    for (entity, source, replicated) in query.iter_mut() {
        if let Some(mut replicated) = replicated {
            *replicated = Replicated(source.clone());
        } else {
            commands.entity(entity).insert(Replicated(source.clone()));
        }
    }
}

pub fn sync_from_replication<T: Clone + Component + Default + Reflect>(
    mut commands: Commands,
    mut query: Query<(Entity, &Replicated<T>, Option<&mut T>)>,
) {
    for (entity, replicated, target) in query.iter_mut() {
        if let Some(mut target) = target {
            *target = replicated.0.clone();
        } else {
            commands.entity(entity).insert(replicated.0.clone());
        }
    }
}

pub struct ReplicationStrategyPlugin;

impl Plugin for ReplicationStrategyPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            // TODO: server event mapping not working?
            .add_server_event::<messages::PlayerAssignmentMessageEvent>(SendPolicy::Unordered)
            .add_mapped_client_event::<messages::ActionMessageEvent>(SendPolicy::Ordered)
            // "static" components that will not be mutated during gameplay
            .replicate::<PlayerID>()
            .replicate::<GameLobby>()
            .replicate::<GameInstance>()
            .register_type::<Identity>()
            .replicate::<PlayerOne>()
            .replicate::<PlayerTwo>()
            .replicate::<FielderTrack>()
            .replicate::<Boundary>()
            .replicate::<Wicket>()
            .register_type::<BowlScore>()
            .register_type::<Vec<BowlScore>>()
            .replicate::<Scoreboard>()
            // dynamic components
            // entity "kinds"
            .replicate::<Position>()
            .replicate::<GamePhase>()
            .register_type::<FielderPosition>()
            .register_type::<FielderRing>()
            .register_type::<Fielder>()
            .replicate::<Replicated<Fielder>>()
            .register_type::<Batter>()
            .replicate::<Replicated<Batter>>()
            .register_type::<Ball>()
            .replicate::<Replicated<Ball>>()
            // physics components
            .register_type::<Transform>()
            .replicate::<Replicated<Transform>>()
            .register_type::<Velocity>()
            .replicate::<Replicated<Velocity>>()
            .register_type::<ExternalImpulse>()
            .replicate::<ExternalImpulse>();
        #[cfg(debug_assertions)]
        app.replicate::<Name>();
    }
}

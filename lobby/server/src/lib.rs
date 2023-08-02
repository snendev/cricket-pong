use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::{
    prelude::SystemSet,
    schedule::{IntoSystemConfigs, IntoSystemSetConfig},
};

pub mod resources;
use naia_bevy_server::ReceiveEvents;
use resources::QueuedUsers;

mod systems;
pub use systems::subscribe_to_game_instances;

pub struct CommonLobbyPlugin;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LobbySet;

impl Plugin for CommonLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::UserEntities>()
            .init_resource::<QueuedUsers>()
            .configure_set(Update, LobbySet.in_set(ReceiveEvents))
            .add_systems(
                Update,
                ((
                    systems::handle_user_connection,
                    systems::handle_user_disconnection,
                    systems::handle_room_cleanup,
                )
                    .in_set(LobbySet),),
            );
    }
}

use bevy_app::{
    prelude::{App, Plugin},
    Update,
};
use bevy_ecs::prelude::IntoSystemConfigs;

pub mod resources;

mod systems;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct NoState;

pub struct CommonLobbyPlugin;

impl Plugin for CommonLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::UserEntityMap>()
            .init_resource::<resources::LobbyStateMap>()
            .init_resource::<resources::ReadiedUsers>()
            .add_systems(
                Update,
                (
                    (
                        systems::handle_room_ready,
                        systems::send_room_start_signal,
                        systems::handle_room_cleanup,
                        systems::flush_room_state_updates,
                    )
                        .chain(),
                    systems::receive_state_update_message,
                    systems::handle_user_join_room,
                    systems::remove_user,
                    systems::subscribe_rooms_to_game_instance,
                ),
            );
    }
}

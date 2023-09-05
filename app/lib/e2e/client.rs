use bevy::prelude::{App, States};

use bevy_geppetto::Test;

use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_app_lib::networking::OnlineGameplayPlugin;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn main() {
    Test {
        label: "Client".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins(OnlineGameplayPlugin::new(TestState::Test))
                .add_plugins(GraphicsPlugin::new(TestState::Complete))
                .add_plugins(PlayerControllerPlugin);
        },
    }
    .run();
}

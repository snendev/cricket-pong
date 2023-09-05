use bevy_app::prelude::App;
use bevy_ecs::prelude::States;

use bevy_geppetto::Test;

use cricket_pong_graphics::GraphicsPlugin;
use cricket_pong_server::ServerPlugin;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Complete,
}

fn main() {
    Test {
        label: "Server".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins(ServerPlugin)
                .add_plugins(GraphicsPlugin::new(TestState::Complete));
        },
    }
    .run();
}

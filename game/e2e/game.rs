use bevy_app::{prelude::App, Startup};
use bevy_ecs::{
    prelude::Commands,
    schedule::{States, SystemSet},
};
use bevy_geppetto::Test;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use cricket_pong_base::{PlayerOne, PlayerTwo, Position, Score};
use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_game::{GamePhase, GameplayPlugin};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct GameplaySet;
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Gameover,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((Position::Batter, PlayerOne, Score(0)));
    commands.spawn((Position::Fielder, PlayerTwo, Score(0)));
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins((
                    RapierDebugRenderPlugin::default(),
                    GraphicsPlugin::new(TestState::Test, TestState::Test, GamePhase::GameOver),
                    GameplayPlugin::new(GameplaySet, TestState::Test),
                    PlayerControllerPlugin,
                ))
                .add_systems(Startup, spawn_players);
        },
    }
    .run();
}

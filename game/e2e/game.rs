use bevy_app::{prelude::App, Startup};
use bevy_ecs::prelude::{Commands, Local, Res, States, SystemSet};

use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy_geppetto::Test;

use cricket_pong_base::components::player::{PlayerOne, PlayerTwo, Position, Score};
use cricket_pong_controls::{Controller, PlayerControllerPlugin};
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_game::{Actions, GamePhase, GameplayPlugin};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct GameplaySet;
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Test,
    Gameover,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Position::Batter,
        PlayerOne,
        Controller::One,
        Score::default(),
    ));
    commands.spawn((
        Position::Fielder,
        PlayerTwo,
        Controller::Two,
        Score::default(),
    ));
}

fn yield_local_ticks(actions: Res<Actions>, mut tick: Local<u16>) -> Vec<(u16, Actions)> {
    let result = (*tick, actions.clone());
    *tick += 1;
    vec![result]
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<TestState>()
                .add_plugins((
                    RapierDebugRenderPlugin::default(),
                    GameplayPlugin::new(GameplaySet, TestState::Test, yield_local_ticks),
                ))
                .add_plugins((
                    GraphicsPlugin::new(TestState::Test, TestState::Test, GamePhase::GameOver),
                    PlayerControllerPlugin,
                ))
                .add_systems(Startup, spawn_players);
        },
    }
    .run();
}

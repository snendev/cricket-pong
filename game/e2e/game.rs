use bevy_app::{prelude::App, Startup};
use bevy_ecs::{
    prelude::Commands,
    schedule::{States, SystemSet},
};
use bevy_geppetto::Test;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use cricket_pong_base::{Objective, Player};
// use cricket_pong_bots::{BatterBotBundle, BotControllerPlugin};
use cricket_pong_controls::{
    BatterControllerBundle2, FielderControllerBundle, PlayerControllerPlugin,
};
use cricket_pong_graphics::GraphicsPlugin;

use cricket_pong_game::GameplayPlugin;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct GameplaySet;
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum GameplayState {
    #[default]
    Test,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Player::new(Objective::Batting),
        BatterControllerBundle2::new(),
    ));
    commands.spawn((
        Player::new(Objective::Fielding),
        FielderControllerBundle::new(),
    ));
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_state::<GameplayState>()
                .add_plugins((
                    RapierDebugRenderPlugin::default(),
                    GraphicsPlugin,
                    GameplayPlugin::new(GameplaySet, GameplayState::Test),
                    PlayerControllerPlugin,
                    // BotControllerPlugin,
                ))
                .add_systems(Startup, spawn_players);
        },
    }
    .run();
}

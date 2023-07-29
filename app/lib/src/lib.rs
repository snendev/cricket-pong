use bevy::prelude::{
    App, Commands, DefaultPlugins, Local, OnEnter, PluginGroup, Res, States, SystemSet, Window,
    WindowPlugin,
};

use cricket_pong_controls::{Controller, PlayerControllerPlugin};
use cricket_pong_game::{
    base::components::player::{PlayerOne, PlayerTwo},
    Actions, GamePhase, GameplayPlugin,
};
use cricket_pong_graphics::GraphicsPlugin;

mod home;
use home::HomeScreenPlugin;

mod networking;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LocalGameplaySet;
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct OnlineGameplaySet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
enum AppScreen {
    Splash,
    #[default]
    MainMenu,
    LocalGame,
    // ** TODO:
    // AIGame,
    OnlineGame,
}

fn spawn_local_players(mut commands: Commands) {
    commands.spawn((PlayerOne, Controller::One));
    commands.spawn((PlayerTwo, Controller::Two));
}

fn yield_local_ticks(actions: Res<Actions>, mut tick: Local<u16>) -> Vec<(u16, Actions)> {
    let result = (*tick, actions.clone());
    *tick += 1;
    vec![result]
}

pub fn run_app(canvas: Option<String>) {
    App::default()
        .add_state::<AppScreen>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(HomeScreenPlugin)
        .add_plugins(GameplayPlugin::new(LocalGameplaySet, yield_local_ticks))
        .add_plugins(networking::NetworkPlugin)
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet,
            networking::send_and_prepare_inputs,
        ))
        .add_plugins(GameplayPlugin::new(
            OnlineGameplaySet,
            networking::receive_update_component_events,
        ))
        .add_plugins((
            PlayerControllerPlugin,
            GraphicsPlugin::new(AppScreen::MainMenu),
        ))
        .add_systems(OnEnter(AppScreen::LocalGame), spawn_local_players)
        .run();
}

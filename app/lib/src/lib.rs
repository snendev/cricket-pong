use bevy::prelude::{
    in_state, App, Commands, DefaultPlugins, IntoSystemSetConfig, OnEnter, PluginGroup, States,
    SystemSet, Update, Window, WindowPlugin,
};

use cricket_pong_controls::{Controller, PlayerControllerPlugin};
use cricket_pong_game::base::components::player::{PlayerOne, PlayerTwo};
use cricket_pong_graphics::GraphicsPlugin;

mod home;
use home::HomeScreenPlugin;

pub mod networking;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LocalGameplaySet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppScreen {
    Splash,
    #[default]
    MainMenu,
    LocalGame,
    // ** TODO:
    // AIGame,
    OnlineGame,
}

fn spawn_local_players(mut commands: Commands) {
    commands.spawn((PlayerOne, PlayerOne::name(), Controller::One));
    commands.spawn((PlayerTwo, PlayerTwo::name(), Controller::Two));
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
        .configure_set(
            Update,
            LocalGameplaySet.run_if(in_state(AppScreen::LocalGame)),
        )
        // .add_plugins(GameplayPlugin::new(LocalGameplaySet, yield_local_ticks))
        .add_plugins(networking::OnlineGameplayPlugin)
        .add_plugins((
            PlayerControllerPlugin,
            GraphicsPlugin::new(AppScreen::MainMenu),
        ))
        .add_systems(OnEnter(AppScreen::LocalGame), spawn_local_players)
        .run();
}

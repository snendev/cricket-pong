use bevy::prelude::{
    in_state, App, Commands, DefaultPlugins, IntoSystemSetConfig, OnEnter, PluginGroup, States,
    SystemSet, Update, Window, WindowPlugin,
};

use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_game::{
    base::{PlayerOne, PlayerTwo, Position, Score},
    GamePhase, GameplayPlugin,
};
use cricket_pong_graphics::GraphicsPlugin;
use home::HomeScreenPlugin;

mod home;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LocalGameplaySet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
enum AppScreen {
    Splash,
    #[default]
    MainMenu,
    LocalGame,
    // ** TODO:
    // AIGame,
    // OnlineGame,
}

fn spawn_local_players(mut commands: Commands) {
    commands.spawn((Position::Batter, PlayerOne, Score(0)));
    commands.spawn((Position::Fielder, PlayerTwo, Score(0)));
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
        .add_plugins(GameplayPlugin::new(LocalGameplaySet, AppScreen::LocalGame))
        .add_plugins((
            PlayerControllerPlugin,
            GraphicsPlugin::new(
                AppScreen::LocalGame,
                AppScreen::MainMenu,
                GamePhase::GameOver,
            ),
        ))
        .add_systems(OnEnter(AppScreen::LocalGame), spawn_local_players)
        .run();
}

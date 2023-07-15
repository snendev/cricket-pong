use bevy::{
    prelude::{
        in_state, App, Commands, IntoSystemSetConfig, OnEnter, PluginGroup, States, SystemSet,
        Update, Window, WindowPlugin,
    },
    DefaultPlugins,
};

// use cricket_pong_bots::BotControllerPlugin;
use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_game::{
    base::{Player, Position},
    GameplayPlugin,
};
use cricket_pong_graphics::GraphicsPlugin;
use home::HomeScreenPlugin;

mod home;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct LocalGameplaySet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, SystemSet)]
pub struct OnlineGameplaySet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
enum AppScreen {
    Splash,
    #[default]
    MainMenu,
    // AIGame,
    LocalGame,
    OnlineGame,
}

fn spawn_local_players(mut commands: Commands) {
    commands.spawn((Position::Batter, Player::new(1.try_into().unwrap())));
    commands.spawn((Position::Fielder, Player::new(2.try_into().unwrap())));
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
        .configure_set(
            Update,
            OnlineGameplaySet.run_if(in_state(AppScreen::OnlineGame)),
        )
        .add_plugins((
            GameplayPlugin::new(LocalGameplaySet, AppScreen::LocalGame),
            // GameplayPlugin::new(OnlineGameplaySet, AppScreen::OnlineGame),
        ))
        .add_plugins((
            PlayerControllerPlugin,
            // BotControllerPlugin,
            GraphicsPlugin,
        ))
        .add_systems(OnEnter(AppScreen::LocalGame), spawn_local_players)
        .run();
}

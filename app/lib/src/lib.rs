use bevy::prelude::{App, DefaultPlugins, PluginGroup, States, Window, WindowPlugin};

use cricket_pong_controls::PlayerControllerPlugin;
use cricket_pong_graphics::GraphicsPlugin;

mod home;
use home::HomeScreenPlugin;

pub mod local;
// pub mod old_networking;
pub mod networking;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
enum AppScreen {
    Splash,
    #[default]
    MainMenu,
    LocalGame,
    OnlineGame,
    // ** TODO:
    // AIGame,
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
        .add_plugins(local::LocalGameplayPlugin::new(AppScreen::LocalGame))
        .add_plugins(networking::OnlineGameplayPlugin::new(AppScreen::OnlineGame))
        .add_plugins((
            PlayerControllerPlugin,
            GraphicsPlugin::new(AppScreen::MainMenu),
        ))
        .run();
}

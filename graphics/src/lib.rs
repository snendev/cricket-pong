use bevy::prelude::{App, Camera2dBundle, Commands, Plugin, Startup, States, SystemSet};

mod objects;
mod ui;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct GraphicsSet;

pub struct GraphicsPlugin<AppScreen: States, GameState: States> {
    active_screen: AppScreen,
    return_screen: AppScreen,
    gameover_state: GameState,
}

impl<AppScreen, GameState> GraphicsPlugin<AppScreen, GameState>
where
    AppScreen: States,
    GameState: States,
{
    pub fn new(
        active_screen: AppScreen,
        return_screen: AppScreen,
        gameover_state: GameState,
    ) -> Self {
        GraphicsPlugin {
            active_screen,
            return_screen,
            gameover_state,
        }
    }
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl<AppScreen, GameState> Plugin for GraphicsPlugin<AppScreen, GameState>
where
    AppScreen: States + Copy,
    GameState: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(objects::ObjectGraphicsPlugin)
            .add_plugins(ui::GameUIPlugin::new(
                self.active_screen,
                self.return_screen,
                self.gameover_state,
            ))
            .add_systems(Startup, setup_camera);
    }
}

use bevy::prelude::{App, Camera2dBundle, Commands, Plugin, Startup, States, SystemSet};

mod objects;
mod ui;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct GraphicsSet;

pub struct GraphicsPlugin<AppScreen: States> {
    return_screen: AppScreen,
}

impl<AppScreen> GraphicsPlugin<AppScreen>
where
    AppScreen: States,
{
    pub fn new(return_screen: AppScreen) -> Self {
        GraphicsPlugin { return_screen }
    }
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl<AppScreen> Plugin for GraphicsPlugin<AppScreen>
where
    AppScreen: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(objects::ObjectGraphicsPlugin)
            .add_plugins(ui::GameUIPlugin::new(self.return_screen))
            .add_systems(Startup, setup_camera);
    }
}

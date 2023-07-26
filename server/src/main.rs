use std::time::Duration;

use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_log::{info, LogPlugin};

use cricket_pong_server::ServerPlugin;

fn main() {
    info!("Starting up cricket pong server...");

    App::default()
        .add_plugins((
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin::default(),
            FrameCountPlugin::default(),
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)),
            LogPlugin::default(),
        ))
        .add_plugins(ServerPlugin)
        .run();
}

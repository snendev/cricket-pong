use bevy::prelude::*;

use bevy_egui::{
    egui::{CentralPanel, Vec2},
    EguiContexts, EguiPlugin,
};

use crate::AppScreen;

fn home_menu(mut egui_ctx: EguiContexts, mut screen_state: ResMut<NextState<AppScreen>>) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_height(300.);
            ui.set_width(200.);
            ui.vertical_centered(|ui| {
                ui.allocate_ui(Vec2::new(200., 80.), |ui| {
                    if ui.button("Play Locally").clicked() {
                        screen_state.set(AppScreen::LocalGame);
                    }
                });
                ui.allocate_ui(Vec2::new(200., 80.), |ui| {
                    if ui.button("Play Online").clicked() {
                        screen_state.set(AppScreen::OnlineGame);
                    }
                });
            });
        });
    });
}

pub struct HomeScreenPlugin;

impl Plugin for HomeScreenPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Update, home_menu.run_if(in_state(AppScreen::MainMenu)));
    }
}

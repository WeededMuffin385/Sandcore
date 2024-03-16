use bevy::prelude::*;
use bevy::app::{App, AppExit, Update};
use bevy_egui::{egui, EguiContexts};
use crate::application::menu::state::MenuState;

pub struct MainMenu;


impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_ui.run_if(in_state(MenuState::MainMenu)))


        ;
    }
}

fn update_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,

    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.heading("Main Menu");
        });
    });

    egui::CentralPanel::default().show(ctx, |ui|{
        let button_size = [ui.available_width() * 0.35, 20.0];

        ui.vertical_centered(|ui|{
            if ui.add_sized(button_size, egui::widgets::Button::new("Multiplayer")).clicked() {

            }

            if ui.add_sized(button_size, egui::widgets::Button::new("Settings")).clicked() {
                next_state.set(MenuState::SettingsMenu);
            }

            if ui.add_sized(button_size, egui::widgets::Button::new("Exit")).clicked() {
                exit.send(AppExit);
            }
        });
    });
}
mod settings;
mod state;

use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, Res, ResMut, State};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Ui;
use settings::Settings;
use crate::application::menu::settings_menu::state::SettingsMenuState;
use crate::application::menu::state::MenuState;

pub struct SettingsMenu;
impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SettingsMenuState>()
            .init_resource::<Settings>()
            .add_systems(OnEnter(MenuState::SettingsMenu), on_enter)
            .add_systems(OnExit(MenuState::SettingsMenu), on_exit)
            .add_systems(Update, update_ui.run_if(in_state(MenuState::SettingsMenu)))
        ;
    }
}

fn on_enter(

){

}

fn on_exit(

) {

}

fn update_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,

    current_state: Res<State<SettingsMenuState>>,
    mut next_state: ResMut<NextState<SettingsMenuState>>,

    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel").exact_width(ctx.available_rect().width() * 0.2).show(ctx, |ui|{
        let button_size = [ui.available_width(), 20.0];

        if ui.add_sized(button_size, egui::Button::new("General")).clicked() {
            next_state.set(SettingsMenuState::General);
        }

        if ui.add_sized(button_size, egui::Button::new("Interface")).clicked() {
            next_state.set(SettingsMenuState::Interface);
        }

        if ui.add_sized(button_size, egui::Button::new("Languages")).clicked() {
            next_state.set(SettingsMenuState::Languages);
        }

        ui.add_space(ui.available_height() - button_size[1]);

        if ui.add_sized(button_size, egui::Button::new("Save'n'back")).clicked() {
            next_menu_state.set(MenuState::MainMenu);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        match current_state.get() {
            SettingsMenuState::General => {
                show_general_settings(ui, &mut settings)
            }
            SettingsMenuState::Interface => {
                ui.label("interface settings");
            }
            SettingsMenuState::Languages => {
                ui.label("languages settings");
            }
        }
    });
}

fn show_general_settings(ui: &mut Ui, settings: &mut ResMut<Settings>) {
    ui.style_mut().spacing.slider_width = ui.available_width() * 0.5;

    ui.horizontal(|ui| {
        let slider = egui::widgets::Slider::new(&mut settings.font_size, 0.0..=60.0).text("font size");
        ui.add(slider);
    });
}
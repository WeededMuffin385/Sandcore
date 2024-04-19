mod renderer;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Entity, in_state, IntoSystemConfigs, Mut, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, State};
use bevy::tasks::AsyncComputeTaskPool;
use bevy_egui::{egui, EguiContexts};
use sandcore_core::message::Message;
use crate::application::gameplay::game::renderer::Renderer;
use crate::application::gameplay::server::ServerResource;
use crate::application::gameplay::state::GameplayState;
use crate::application::menu::state::MenuState;
use crate::application::state::ApplicationState;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Renderer)
            .add_systems(OnEnter(GameplayState::Game), on_enter)
            .add_systems(OnExit(GameplayState::Game), on_exit)
            .add_systems(Update, (
                update_ui,
                update_echo,
                update_server,
            )
            .run_if(in_state(ApplicationState::Gameplay))
            .run_if(in_state(GameplayState::Game)));
    }
}

struct Creature;

fn on_enter(
) {
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async { println!("hello world from on_enter") });
}

fn on_exit(
) {
}

fn update_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,

    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel").exact_width(ctx.available_rect().width() * 0.2).show(ctx, |ui|{
        ui.add_space(ui.available_height() - 20.0);

        ui.vertical_centered_justified(|ui|{
            if ui.button("disconnect").clicked() {
                commands.remove_resource::<ServerResource>();

                next_menu_state.set(MenuState::MainMenu);
                next_application_state.set(ApplicationState::Menu);
            }
        });

        ui.label("some text");
    });
}

fn update_server(
    //mut server: ResMut<ServerResource>,

){
    println!("updated server!");


}

fn update_echo(
) {
    println!("echoed");
}
use bevy::{prelude::*, ecs::system::SystemParam};
use bevy_egui::{
    egui::{self, Color32 },
    EguiContext,
};

use super::{user_api::{UserDto, AuthTask}, login_manager::{continue_unregistered_screen, create_account_screen}};
use super::login_manager::login_screen;
use super::user_api;
use super::login_manager::UserCreate;
use super::ui::UiState;

#[derive(SystemParam)]
pub struct LoginContext<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub user_choice: ResMut<'w, UserChoice>,
    pub user_dto: ResMut<'w, UserDto>,
    pub user_create: ResMut<'w, UserCreate>,
}

#[derive(Resource)]
pub enum UserChoice {
    None,
    Unregistered,
    CreateAccount,
    Login,
}

impl Default for UserChoice {
    fn default() -> Self {
        UserChoice::None
    }
}

// A barebones egui for onboarding to the game
pub fn onboarding_ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut login_context: LoginContext,
    mut auth_tasks: Query<(Entity, &mut AuthTask)>,
) {
    egui::SidePanel::left("left_panel")
        .min_width(400.)
        .max_width(400.)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Lobby");
            ui.separator();

            display_errors_if_present(ui, ui_state.as_ref());

            user_api::poll_tasks(&mut login_context.commands, &mut ui_state, &mut auth_tasks, 
                                 login_context.user_dto.as_mut());

            match login_context.user_choice.as_mut() {
                UserChoice::Login => {
                    login_screen(ui, &mut login_context, &mut ui_state);
                }
                UserChoice::CreateAccount => {
                    create_account_screen(ui, &mut login_context, &mut ui_state)
                }
                UserChoice::Unregistered => {
                    continue_unregistered_screen(ui, &mut login_context, &mut ui_state);
                }
                UserChoice::None => {
                    ui.vertical(|ui| {
                        if ui.button("Login").clicked() {
                            *login_context.user_choice = UserChoice::Login;
                        }
                        ui.separator();
                        if ui.button("Create An Account").clicked() {
                            *login_context.user_choice = UserChoice::CreateAccount;
                        }
                        ui.separator();
                        if ui.button("Continue as Unregistered").clicked() {
                            *login_context.user_choice = UserChoice::Unregistered;
                        }
                    });
                }
            }
        });
}

fn display_errors_if_present(ui: &mut egui::Ui, ui_state: &UiState) {
    if !ui_state.error.is_empty() {
        ui.separator();
        ui.colored_label(Color32::from_rgb(255, 0, 0), &ui_state.error);
        ui.separator();
    }
}

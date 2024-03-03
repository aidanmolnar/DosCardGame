use bevy::prelude::Resource;
use bevy_egui::egui::{TextEdit, self};
use iyes_loopless::state::NextState;
use serde::Serialize;

use super::{user_api::{UserDto, self}, onboarding_ui::{UserChoice, LoginContext}, ui::UiState, OnboardingState};

#[derive(Serialize, Resource)]
pub struct UserCreate {
    username: String,
    password: String,
    confirm_password: String,
}

impl Default for UserCreate {
    fn default() -> Self {
        UserCreate {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
        }
    }
}

pub fn login_screen(ui: &mut egui::Ui, 
                    login_context: &mut LoginContext,
                    ui_state: &mut UiState) {
    display_login_screen(ui, login_context.user_dto.as_mut());
    if ui.button("Login").clicked() {
        user_api::spawn_task(&mut login_context.commands, login_context.user_dto.clone(), 
                             user_api::login_user);
    } else if ui.button("Back").clicked() {
        ui_state.error.clear();
        *login_context.user_choice = UserChoice::None;
    }
}

pub fn create_account_screen(ui: &mut egui::Ui, 
                             login_context: &mut LoginContext,
                             ui_state: &mut UiState) {
    display_create_user_screen(ui, &mut login_context.user_create);
    if ui.button("Create Account").clicked() {
        handle_user_create_input(ui_state, login_context);
        if ui_state.error.is_empty() {
            user_api::spawn_task(&mut login_context.commands, login_context.user_dto.clone(), 
                                 user_api::create_user_and_login);
        }
    } else if ui.button("Back").clicked() {
        ui_state.error.clear();
        *login_context.user_choice = UserChoice::None;
    }
}

pub fn continue_unregistered_screen(ui: &mut egui::Ui,
                                    login_context: &mut LoginContext,
                                    ui_state: &mut UiState) {
    display_unregistered_screen(ui, &mut login_context.user_dto);
    if ui.button("Continue").clicked() {
        let username = login_context.user_dto.username.clone();
        if username.len() > 20 {
            ui_state.error = "Username is too long".to_string();
        } else if username.is_empty() {
            ui_state.error = "Username is too short".to_string();
        } 
        else {
            ui_state.error.clear();
            ui_state.name = login_context.user_dto.username.clone();
            login_context.commands.insert_resource(NextState(OnboardingState::Authenticated));
        }
    } else if ui.button("Back").clicked() {
        ui_state.error.clear();
        *login_context.user_choice = UserChoice::None;
    }
}

fn handle_user_create_input(ui_state: &mut UiState, 
                            login_context: &mut LoginContext) {
    let user_input = &login_context.user_create;
    if user_input.password != login_context.user_create.confirm_password {
        ui_state.error = "Passwords must match".to_string();
    } else if user_input.password.len() < 8 {
        ui_state.error = "Password should be at least 8 characters".to_string();
    } else {
        login_context.user_dto.username = user_input.username.clone();
        login_context.user_dto.password = user_input.password.clone();
        ui_state.error.clear();
    }
}

fn display_login_screen(ui: &mut egui::Ui, user_dto: &mut UserDto) {
    ui.horizontal(|ui| {
        ui.label("Username: ");
        TextEdit::singleline(&mut user_dto.username)
            .hint_text("Please enter a Username")
            .desired_width(f32::INFINITY)
            .show(ui);
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Password: ");
        TextEdit::singleline(&mut user_dto.password)
            .password(true)
            .hint_text("Please enter a Password")
            .desired_width(f32::INFINITY)
            .show(ui);
    });
    ui.separator();
}

fn display_unregistered_screen(ui: &mut egui::Ui, user_dto: &mut UserDto) {
    ui.horizontal(|ui| {
        ui.label("Username: ");
        TextEdit::singleline(&mut user_dto.username)
            .hint_text("Please enter a Username")
            .desired_width(f32::INFINITY)
            .show(ui);
    });
}

fn display_create_user_screen(ui: &mut egui::Ui, user_create: &mut UserCreate) {
    ui.horizontal(|ui| {
        ui.label("Username: ");
        TextEdit::singleline(&mut user_create.username)
            .desired_width(f32::INFINITY)
            .show(ui);
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Password: ");
        TextEdit::singleline(&mut user_create.password)
            .password(true)
            .desired_width(f32::INFINITY)
            .show(ui);
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Confirm Password: ");
        TextEdit::singleline(&mut user_create.confirm_password)
            .password(true)
            .desired_width(f32::INFINITY)
            .show(ui);
    });
}

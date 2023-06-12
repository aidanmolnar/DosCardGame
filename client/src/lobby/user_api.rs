use bevy::{prelude::{Resource, Commands, Component, ResMut, Query, Entity}, tasks::{AsyncComputeTaskPool, Task}};
use futures_lite::future;
use iyes_loopless::state::NextState;
use serde::{Serialize, Deserialize};

use crate::lobby::OnboardingState;

use super::ui::UiState;

const BASE_URL: &str = "https://dos-go-user-api.alwaysdata.net";
const LOGIN_ENDPOINT: &str = "/api/login";
const USER_CREATE_ENDPOINT: &str = "/api/users";

#[derive(Serialize, Resource, Clone)]
pub struct UserDto {
    pub username: String,
    pub password: String,
}

#[derive(Component)]
pub struct AuthTask(pub Task<Result<AuthToken, reqwest::Error>>);

impl Default for UserDto {
    fn default() -> Self {
        UserDto {
            username: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthToken {
    pub session: String,
}

pub fn login_user(user: &UserDto) -> Result<AuthToken, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = [BASE_URL, LOGIN_ENDPOINT].join("\n");
    let res = client.post(url) 
        .json(user) 
        .send()?;
    return res.json::<AuthToken>();
}

pub fn create_user_and_login(user: &UserDto) 
                -> Result<AuthToken, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = [BASE_URL, USER_CREATE_ENDPOINT].join("\n");
    client.post(url)
        .json(user)
        .send()?;

    return login_user(user);
}

pub fn spawn_task<F>(commands: &mut Commands, 
                  user: UserDto, 
                  block_fn: F) where F: Fn(&UserDto) -> Result<AuthToken, reqwest::Error> + Send + 'static,
{
    let thread_pool = AsyncComputeTaskPool::get();

    let auth_task = thread_pool.spawn(async move { 
        block_fn(&user)
    });
    
    commands.spawn(AuthTask(auth_task));
}

pub fn poll_tasks(commands: &mut Commands,
                  ui_state: &mut ResMut<UiState>,
                  auth_tasks: &mut Query<(Entity, &mut AuthTask)>,
                  user_dto: &UserDto) {
    for (entity, mut task) in auth_tasks {
        if let Some(token) = future::block_on(future::poll_once(&mut task.0)) {
            match token {
                Ok(_auth_token) => {
                    ui_state.error.clear();
                    ui_state.name = user_dto.username.clone();
                    commands.insert_resource(NextState(OnboardingState::Authenticated));
                }
                Err(error) => {
                    eprintln!("Failed to retrieve auth token {:?}", error);
                    ui_state.error = "Credentials are invalid.".into();
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

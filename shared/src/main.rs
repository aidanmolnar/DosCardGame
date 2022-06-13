use bevy::prelude::*;

mod base;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(start)
    .run();
}

fn start() {
    let mut game = base::new_game(2);

    game.deal_in_players();

    println!("{:?}", game);

    println!("Starting Game");
}
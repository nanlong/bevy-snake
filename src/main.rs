use bevy::prelude::*;
use bevy_snake::{food::FoodPlugin, snake::SnakePlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(FoodPlugin)
        .run();
}

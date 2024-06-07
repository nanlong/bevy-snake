use bevy::prelude::*;
use bevy_snake::{food::FoodPlugin, snake::SnakePlugin, world::WorldPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: (500., 500.).into(),
                ..default()
            }),

            ..default()
        }))
        .add_plugins(WorldPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(FoodPlugin)
        .run();
}

use crate::grid::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::seq::IteratorRandom;
use std::time::Duration;

const FOOD_COLOR: Color = Color::rgb(1., 0., 1.);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_food.run_if(on_timer(Duration::from_secs(1))));
    }
}

#[derive(Component)]
pub struct Food;

fn spawn_food(mut commands: Commands, positions: Query<&Position>) {
    let all_positions = gen_all_positions();

    let position = all_positions
        .iter()
        .filter(|pos| !positions.iter().any(|p| p == *pos))
        .choose(&mut rand::thread_rng());

    if let Some(position) = position {
        let food = (
            SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            },
            *position,
            Size::square(0.8),
            Food,
        );

        commands.spawn(food);
    }
}

fn gen_all_positions() -> Vec<Position> {
    let mut positions = Vec::new();

    for x in 0..ARENA_WIDTH {
        for y in 0..ARENA_HEIGHT {
            positions.push(Position {
                x: x as i32,
                y: y as i32,
            });
        }
    }

    positions
}

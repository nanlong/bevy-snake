use crate::grid::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (size_scaling, position_translation));
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera);
}

fn size_scaling(windows: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.single();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.0,
        );
    }
}

fn position_translation(windows: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.single();

    let convert = |pos: f32, bound_window: f32, bound_game: f32| {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - bound_window / 2.0 + tile_size / 2.0
    };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

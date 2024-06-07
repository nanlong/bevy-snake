use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::{
    food::Food,
    grid::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::{prelude::*, time::common_conditions::on_timer, utils::info};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .insert_resource(LastInputDirection::default())
            .add_systems(Startup, spawn_snake)
            .add_systems(
                Update,
                snake_movement.run_if(on_timer(Duration::from_millis(150))),
            )
            .add_systems(Update, game_over.after(snake_movement))
            .add_systems(Update, snake_eating.after(snake_movement))
            .add_systems(Update, snake_growth.after(snake_eating))
            .add_systems(Update, snake_movement_input.before(snake_movement))
            .add_systems(Update, snake_direction.after(snake_movement_input))
            .add_event::<GrowthEvent>()
            .add_event::<GameOverEvent>();
    }
}

#[derive(Event)]
struct GrowthEvent;

#[derive(Event)]
struct GameOverEvent;

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
    moved: bool,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Resource, Default)]
pub struct SnakeSegments(Vec<Entity>);

impl Deref for SnakeSegments {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SnakeSegments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Default)]
pub struct LastTailPosition(Option<Position>);

#[derive(Resource, Default)]
pub struct LastInputDirection(Option<Direction>);

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    let snake_head = (
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        },
        Position { x: 3, y: 3 },
        Size::square(0.8),
        SnakeHead {
            direction: Direction::Up,
            moved: false,
        },
    );

    // 生成一个头部
    let snake_head_id = commands.spawn(snake_head).id();
    // 生成一个小尾巴
    let snake_segment_id = spawn_segment(commands, Position { x: 3, y: 2 });

    // 将头部和尾部元素的 ID 存储到资源中
    *segments = SnakeSegments(vec![snake_head_id, snake_segment_id]);
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    let snake_segment = (
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        },
        position,
        Size::square(0.65),
        SnakeSegment,
    );

    commands.spawn(snake_segment).id()
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    heads: Query<&SnakeHead>,
    mut last_input_direction: ResMut<LastInputDirection>,
) {
    let head = heads.single();

    let dir = [
        (KeyCode::KeyA, Direction::Left),
        (KeyCode::KeyD, Direction::Right),
        (KeyCode::KeyW, Direction::Up),
        (KeyCode::KeyS, Direction::Down),
        (KeyCode::ArrowLeft, Direction::Left),
        (KeyCode::ArrowRight, Direction::Right),
        (KeyCode::ArrowUp, Direction::Up),
        (KeyCode::ArrowDown, Direction::Down),
    ]
    .iter()
    .find_map(|(key, dir)| keyboard_input.pressed(*key).then(|| *dir))
    .unwrap_or(head.direction);

    *last_input_direction = LastInputDirection(Some(dir));
}

fn snake_direction(
    mut heads: Query<&mut SnakeHead>,
    last_input_direction: Res<LastInputDirection>,
) {
    let mut head = heads.single_mut();

    if let Some(dir) = last_input_direction.0 {
        if head.moved && dir != head.direction && dir != head.direction.opposite() {
            head.direction = dir;
            head.moved = false;
        }
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    // 获取所有尾部元素的位置信息
    let segment_positions = segments
        .iter()
        .map(|segment_id| *positions.get(segment_id.to_owned()).unwrap())
        .collect::<Vec<_>>();
    let (head_id, mut head) = heads.single_mut();
    // 获取头部位置信息
    let mut head_pos = positions.get_mut(head_id).unwrap();

    // 根据方向更新头部位置
    match head.direction {
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
    }

    head.moved = true;

    // 撞墙检测
    if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 > ARENA_WIDTH
        || head_pos.y as u32 > ARENA_HEIGHT
    {
        game_over_writer.send(GameOverEvent);
    }

    // 撞尾巴检测
    if segment_positions.contains(&head_pos) {
        info("撞到自己了");
        game_over_writer.send(GameOverEvent);
    }

    // 更新所有尾部元素的位置为前一个元素的位置
    segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment_id)| {
            let mut segment_pos = positions.get_mut(*segment_id).unwrap();
            *segment_pos = *pos;
        });

    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    let head_pos = head_positions.single();

    food_positions.iter().for_each(|(food_id, food_pos)| {
        if head_pos == food_pos {
            commands.entity(food_id).despawn();
            growth_writer.send(GrowthEvent);
        }
    });
}

fn snake_growth(
    commands: Commands,
    last_tail_position: ResMut<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        let position = last_tail_position.0.unwrap();
        let segment_id = spawn_segment(commands, position);
        segments.push(segment_id);
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    foods: Query<Entity, With<Food>>,
    headers: Query<Entity, With<SnakeHead>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.read().next().is_some() {
        // 清除所有元素
        for id in foods.iter().chain(headers.iter()).chain(segments.iter()) {
            commands.entity(id).despawn();
        }

        // 重新生成蛇
        spawn_snake(commands, segments_res);
    }
}

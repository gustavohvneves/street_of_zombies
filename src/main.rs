mod game_entity;
mod game_system;
mod weapons;

use bevy::{
    prelude::*
};


use crate::game_entity::*;
use crate::game_system::*;


// Game area limit
static GAME_AREA_LIMIT_X: f32 = 500.0;
static GAME_AREA_LIMIT_Y: f32 = 300.0;


// Main character initialization
static INITIAL_PLAYER_POSITION: f32 = 0.0;
static INITIAL_PLAYER_POSITION_Y: f32 = -215.0;
static INITIAL_PLAYER_SPEED: f32 = 200.0;
static INITIAL_PLAYER_DIRECTION: (f32, f32) = (0.0, 1.0);

static INITIAL_ENNEMY_POSITION_X: f32 = 0.0;
static INITIAL_ENNEMY_POSITION_Y: f32 = 215.0;
static INITIAL_ENNEMY_SPEED: f32 = 200.0;
static INITIAL_ENNEMY_DIRECTION: (f32, f32) = (1.0, 0.0);



fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: GAME_AREA_LIMIT_X,
            height: GAME_AREA_LIMIT_Y,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_title.system())
        .add_system(player::keyboard_capture.system())
        .add_system(projectile_movement_system.system())
        .add_system(projectile_collision_system.system())
        .add_system(ennemy_ai_system.system())
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // Main character
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
            transform: Transform::from_xyz(INITIAL_PLAYER_POSITION, INITIAL_PLAYER_POSITION_Y, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(game_entity::player::Player::new(INITIAL_PLAYER_SPEED, INITIAL_PLAYER_DIRECTION, (INITIAL_PLAYER_POSITION, INITIAL_PLAYER_POSITION_Y)));
    // Ennemy
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.3).into()),
        transform: Transform::from_xyz(INITIAL_ENNEMY_POSITION_X, INITIAL_ENNEMY_POSITION_Y, 0.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(game_entity::ennemies::Ennemy::new(INITIAL_ENNEMY_SPEED, INITIAL_ENNEMY_DIRECTION, (INITIAL_ENNEMY_POSITION_X, INITIAL_ENNEMY_POSITION_Y)));
}

/// This system will then change the title during execution
fn set_window_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
}

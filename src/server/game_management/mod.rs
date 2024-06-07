use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, default, KeyCode, Query, Res, Sprite, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::server::Entity::Entity;
use crate::server::Entity::MovingObject::MovingObject;
use crate::server::Entity::MovingObject::ship::ship;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const BOUNDARY_THICKNESS: f32 = 5.0;
const BOUNDARY_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const BOUNDARY_WIDTH: f32 = 700.0;
const BOUNDARY_HEIGHT: f32 = 500.0;

#[derive(Component)]
pub struct Boundary;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("images/tank.png"),
                ..default()
            },
            ship {
                body: MovingObject {
                    entity: Entity {
                        world_position: [window.width() / 2.0, window.height() / 2.0],
                        world_angle: 0.0,
                        size: PLAYER_SIZE,
                        lock: false,
                        name: "player".to_string(),
                        color: Color::rgb(0.0, 1.0, 0.0),
                    }
                },
                right_wheel: 0.0,
                left_wheel: 0.0,
                gun_orientation: [0.0, 0.0],
            }
        ));
}

pub fn block_players_in_bound(
    mut player_query: Query<&mut Transform, With<ship>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = (window.width() - BOUNDARY_WIDTH) / 2.0 + half_player_size;
        let x_max = (window.width() + BOUNDARY_WIDTH) / 2.0 - half_player_size;
        let y_min = (window.height() - BOUNDARY_HEIGHT) / 2.0 + half_player_size;
        let y_max = (window.height() + BOUNDARY_HEIGHT) / 2.0 - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn player_mov(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<ship>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_boundaries(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_thickness = BOUNDARY_THICKNESS / 2.0;
    let x_offset = (window.width() - BOUNDARY_WIDTH) / 2.0;
    let y_offset = (window.height() - BOUNDARY_HEIGHT) / 2.0;

    // Bottom boundary
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(BOUNDARY_WIDTH, BOUNDARY_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, y_offset + half_thickness, 0.0),
            ..default()
        },
        Boundary,
    ));

    // Top boundary
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(BOUNDARY_WIDTH, BOUNDARY_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - y_offset - half_thickness, 0.0),
            ..default()
        },
        Boundary,
    ));

    // Left boundary
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(BOUNDARY_THICKNESS, BOUNDARY_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x_offset + half_thickness, window.height() / 2.0, 0.0),
            ..default()
        },
        Boundary,
    ));

    // Right boundary
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(BOUNDARY_THICKNESS, BOUNDARY_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() - x_offset - half_thickness, window.height() / 2.0, 0.0),
            ..default()
        },
        Boundary,
    ));
}

pub fn resize_boundaries(
    mut boundary_query: Query<(&mut Transform, &mut Sprite), With<Boundary>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_thickness = BOUNDARY_THICKNESS / 2.0;
    let x_offset = (window.width() - BOUNDARY_WIDTH) / 2.0;
    let y_offset = (window.height() - BOUNDARY_HEIGHT) / 2.0;

    for (mut transform, mut sprite) in boundary_query.iter_mut() {
        // Adjust the positions and sizes of the boundaries based on their original positions
        if sprite.custom_size.unwrap().x == BOUNDARY_WIDTH {
            // Top and Bottom boundaries
            transform.translation.x = window.width() / 2.0;
            if transform.translation.y < window.height() / 2.0 {
                // Bottom boundary
                transform.translation.y = y_offset + half_thickness;
            } else {
                // Top boundary
                transform.translation.y = window.height() - y_offset - half_thickness;
            }
        } else {
            // Left and Right boundaries
            transform.translation.y = window.height() / 2.0;
            if transform.translation.x < window.width() / 2.0 {
                // Left boundary
                transform.translation.x = x_offset + half_thickness;
            } else {
                // Right boundary
                transform.translation.x = window.width() - x_offset - half_thickness;
            }
        }
    }
}

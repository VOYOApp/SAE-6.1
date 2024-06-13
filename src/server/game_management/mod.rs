use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, default, KeyCode, Query, Res, Sprite, SpriteBundle, Time, Transform, Window, With, TextBundle, Text, TextStyle, Font, Handle};
use bevy::window::PrimaryWindow;
use bevy::text::Text2dBundle;

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

struct ShipResource {
    ship: ship,
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let player_texture = asset_server.load("images/tank.png");
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");  // Ensure this path is correct

    // Spawn player ship
    let ship_entity = commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: player_texture.clone(),
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
                right_wheel: 0.5,
                left_wheel: 0.5,
                gun_orientation: [0.0, 0.0],
            }
        ))
        .id();

    // Spawn text for the player ship name
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "player",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0 + PLAYER_SIZE / 2.0 + 15.0, 1.0),
            ..default()
        },
        PlayerNameTag { ship_entity },
    ));
}

#[derive(Component)]
pub struct PlayerNameTag {
    pub ship_entity: bevy::prelude::Entity,
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
    mut query: Query<(&mut ship, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok((mut ship, mut transform)) = query.get_single_mut() {
        ship.update_wheels(keyboard_input);

        let delta_seconds = time.delta_seconds();
        let rotation_speed = std::f32::consts::PI;
        let forward_speed = 500.0;

        if ship.left_wheel == 1.0 && ship.right_wheel == 1.0 {
            // Move forward
            let forward = transform.rotation * Vec3::Y;
            transform.translation += forward * forward_speed * delta_seconds;
        } else if ship.left_wheel == 0.0 && ship.right_wheel == 0.0 {
            // Move backward
            let backward = transform.rotation * Vec3::Y;
            transform.translation -= backward * forward_speed * delta_seconds;
        } else if ship.left_wheel == 1.0 && ship.right_wheel == 0.5 {
            // Turn right
            transform.rotate(Quat::from_rotation_z(-rotation_speed * delta_seconds));
        } else if ship.right_wheel == 1.0 && ship.left_wheel == 0.5 {
            // Turn left
            transform.rotate(Quat::from_rotation_z(rotation_speed * delta_seconds));
        }
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

pub fn update_name_tag_position(
    mut name_query: Query<(&PlayerNameTag, &mut Transform)>,
    ship_query: Query<&Transform, With<ship>>,
) {
    for (name_tag, mut name_transform) in name_query.iter_mut() {
        if let Ok(ship_transform) = ship_query.get(name_tag.ship_entity) {
            name_transform.translation.x = ship_transform.translation.x;
            name_transform.translation.y = ship_transform.translation.y + PLAYER_SIZE / 2.0 + 15.0;
        }
    }
}


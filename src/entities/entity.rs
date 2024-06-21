use rand::Rng;
use rapier2d::prelude::*;
use std::time::Instant;
use eframe::egui;
use crate::physics::physics::PhysicsEngine;

/// Represents an entity in the physics simulation.
pub struct Entity {
    pub name: String,
    pub score: i32,
    pub handle: RigidBodyHandle,
    pub is_ai: bool,
    pub last_shot: Instant,
    pub x: f32,
    pub y: f32,
    pub self_orientation: f64,
    pub gun_orientation: f64,
    pub target_x: f32,
    pub target_y: f32,
    pub color: egui::Color32,
}

impl Entity {
    /// Creates a new `Entity`.
    ///
    /// # Parameters
    /// - `name`: The name of the entity.
    /// - `physics_engine`: A mutable reference to the physics engine.
    /// - `is_ai`: A boolean indicating whether the entity is controlled by AI.
    ///
    /// # Returns
    /// A new instance of `Entity`.
    ///
    /// # Examples
    /// ```
    /// let entity = Entity::new("Player1".to_string(), &mut physics_engine, false);
    /// ```
    pub fn new(name: String, physics_engine: &mut PhysicsEngine, is_ai: bool) -> Self {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(10.0..1190.0);
        let random_y = rng.gen_range(10.0..990.0);
        let vx = rng.gen_range(-100.0..100.0);
        let vy = rng.gen_range(-100.0..100.0);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![random_x, random_y])
            .linvel(vector![vx, vy])
            .build();
        let collider = ColliderBuilder::cuboid(10.0, 10.0)
            .restitution(0.0)
            .build();

        let handle = physics_engine.bodies.insert(rigid_body);
        physics_engine.colliders.insert_with_parent(collider, handle, &mut physics_engine.bodies);

        Self {
            name,
            score: 0,
            handle,
            is_ai,
            last_shot: Instant::now(),
            x: random_x,
            y: random_y,
            self_orientation: 0.0,
            gun_orientation: 0.0,
            target_x: random_x,
            target_y: random_y,
            color: egui::Color32::GREEN,
        }
    }
}

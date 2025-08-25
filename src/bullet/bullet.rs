use rapier2d::prelude::*;
use std::time::Instant;
use crate::physics::physics::PhysicsEngine;

/// Represents a bullet in the physics simulation.
pub struct Bullet {
    pub handle: RigidBodyHandle,
    pub shooter: RigidBodyHandle,
    pub created_at: Instant,
}

impl Bullet {
    /// Creates a new `Bullet`.
    ///
    /// # Parameters
    /// - `shooter_handle`: The handle of the shooter entity.
    /// - `physics_engine`: A mutable reference to the physics engine.
    /// - `speed`: The speed of the bullet.
    /// - `radius`: The radius of the bullet's collider.
    ///
    /// # Returns
    /// A new instance of `Bullet`.
    ///
    /// # Examples
    /// ```
    /// let bullet = Bullet::new(shooter_handle, &mut physics_engine, speed, radius);
    /// ```
    pub fn new(shooter_handle: RigidBodyHandle, physics_engine: &mut PhysicsEngine, speed: f32, radius: f32) -> Self {
        let shooter_body = &physics_engine.bodies[shooter_handle];
        let pos = shooter_body.translation().clone();
        let angle = shooter_body.rotation().angle();
        let direction = vector![angle.cos(), angle.sin()];

        let offset_distance = 20.0;
        let start_pos = pos + direction * offset_distance;

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(start_pos)
            .linvel(direction * speed)
            .build();

        let collider = ColliderBuilder::ball(radius)
            .restitution(0.0)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();

        let handle = physics_engine.bodies.insert(rigid_body);
        physics_engine.colliders.insert_with_parent(collider, handle, &mut physics_engine.bodies);

        Self {
            handle,
            shooter: shooter_handle,
            created_at: Instant::now(),
        }
    }
}

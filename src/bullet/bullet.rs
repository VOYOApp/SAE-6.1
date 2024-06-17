use rapier2d::prelude::*;
use crate::physics::physics::PhysicsEngine;

pub struct Bullet {
    pub handle: RigidBodyHandle,
    pub shooter: RigidBodyHandle,
}

impl Bullet {
    pub fn new(shooter_handle: RigidBodyHandle, physics_engine: &mut PhysicsEngine, line_thickness: f32) -> Self {
        let shooter_body = &physics_engine.bodies[shooter_handle];
        let pos = shooter_body.translation();
        let angle = shooter_body.rotation().angle();
        let direction = vector![angle.cos(), angle.sin()];

        let handle = physics_engine.bodies.insert(
            RigidBodyBuilder::dynamic()
                .translation(*pos)
                .linvel(direction * 500.0)
                .build(),
        );
        let collider = ColliderBuilder::ball(line_thickness).build();
        physics_engine.colliders.insert_with_parent(
            collider,
            handle,
            &mut physics_engine.bodies,
        );

        Self {
            handle,
            shooter: shooter_handle,
        }
    }
}

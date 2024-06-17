use rand::Rng;
use rapier2d::prelude::*;

use crate::physics::physics::PhysicsEngine;

pub struct Entity {
    pub name: String,
    pub score: i32,
    pub handle: RigidBodyHandle,
    pub is_ai: bool,
}

impl Entity {
    pub fn new(name: String, physics_engine: &mut PhysicsEngine, is_ai: bool) -> Self {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(10.0..1190.0);
        let random_y = rng.gen_range(10.0..990.0);

        let handle = physics_engine.bodies.insert(
            RigidBodyBuilder::kinematic_position_based()
                .translation(vector![random_x, random_y])
                .build(),
        );
        let collider = ColliderBuilder::cuboid(10.0, 10.0)
            .restitution(0.0)
            .build();
        physics_engine.colliders.insert_with_parent(
            collider,
            handle,
            &mut physics_engine.bodies,
        );

        Self {
            name,
            score: 0,
            handle,
            is_ai, // Initialize the AI flag
        }
    }
}

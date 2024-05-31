use rand::Rng;
use rapier2d::prelude::*;
use crate::physics::physics::PhysicsEngine;

pub struct Ball {
    pub handle: RigidBodyHandle,
}

pub fn create_balls(engine: &mut PhysicsEngine, count: usize) -> Vec<Ball> {
    let mut rng = rand::thread_rng();
    let mut balls = Vec::new();

    for _ in 0..count {
        let x = rng.gen_range(50.0..1150.0);
        let y = rng.gen_range(50.0..950.0);
        let vx = rng.gen_range(-50.0..50.0);
        let vy = rng.gen_range(-50.0..50.0);
        let ball_handle = engine.bodies.insert(
            RigidBodyBuilder::dynamic()
                .translation(vector![x, y])
                .linvel(vector![vx, vy])
                .build(),
        );
        let ball_collider = ColliderBuilder::ball(10.0).restitution(-2.0).build();
        engine.colliders.insert_with_parent(ball_collider, ball_handle, &mut engine.bodies);
        balls.push(Ball { handle: ball_handle });
    }

    balls
}

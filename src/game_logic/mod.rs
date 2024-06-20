use std::time::Instant;

use rand::Rng;
use rapier2d::prelude::*;

use crate::bullet::bullet::Bullet;
use crate::entities::entity::Entity;
use crate::obstacles::Obstacle;
use crate::physics::physics::PhysicsEngine;

pub struct GameLogic {
    pub physics_engine: PhysicsEngine,
    pub entities: Vec<Entity>,
    pub bullets: Vec<Bullet>,
    pub obstacles: Vec<Obstacle>,
}

impl GameLogic {
    pub fn new() -> Self {
        let mut physics_engine = PhysicsEngine::default();
        physics_engine.setup_boundaries();

        Self {
            physics_engine,
            entities: Vec::new(),
            bullets: Vec::new(),
            obstacles: Vec::new(),
        }
    }

    fn generate_obstacles(&mut self) {
        let mut rng = rand::thread_rng();
        self.obstacles.clear();

        for _ in 0..25 {
            let random_x = rng.gen_range(10.0..1190.0) as f64;
            let random_y = rng.gen_range(10.0..990.0) as f64;

            let collider = ColliderBuilder::cuboid(10.0, 10.0)
                .translation(vector![random_x as f32, random_y as f32])
                .build();
            let collider_handle = self.physics_engine.colliders.insert(collider);

            self.obstacles.push(Obstacle::new((random_x, random_y), collider_handle));
        }
    }

    pub fn add_entity(&mut self, name: String) {
        let entity = Entity::new(name, &mut self.physics_engine, false);
        self.entities.push(entity);
    }

    pub fn shoot_ball(&mut self, shooter_index: usize) {
        if shooter_index >= self.entities.len() {
            return;
        }

        let shooter = &self.entities[shooter_index];
        if shooter.last_shot.elapsed().as_secs_f64() < 1.0 {
            return;
        }

        let bullet = Bullet::new(
            shooter.handle,
            &mut self.physics_engine,
            500.0,  // speed
            5.0     // radius
        );

        self.bullets.push(bullet);
        self.entities[shooter_index].last_shot = Instant::now();
    }

    pub fn step(&mut self) {
        self.physics_engine.step();
        self.remove_out_of_bounds_bullets();

        // Handle bullet collision with entities
        let mut bullet_indices_to_remove = Vec::new();

        for event in &self.physics_engine.collision_events {
            match event {
                CollisionEvent::Started(collider1, collider2, _) => {
                    let body1 = self.physics_engine.colliders[*collider1].parent();
                    let body2 = self.physics_engine.colliders[*collider2].parent();

                    if let (Some(body1), Some(body2)) = (body1, body2) {
                        for (bullet_index, bullet) in self.bullets.iter().enumerate() {
                            if bullet.handle == body1 || bullet.handle == body2 {
                                bullet_indices_to_remove.push(bullet_index);
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        bullet_indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for &index in &bullet_indices_to_remove {
            let bullet = &self.bullets[index];
            self.physics_engine.bodies.remove(
                bullet.handle,
                &mut self.physics_engine.islands,
                &mut self.physics_engine.colliders,
                &mut self.physics_engine.impulse_joints,
                &mut self.physics_engine.multibody_joints,
                true,
            );
            self.bullets.remove(index);
        }
    }

    // Add method to handle out-of-bounds bullets
    pub fn remove_out_of_bounds_bullets(&mut self) {
        let bounds = 1200.0; // Example boundary limit
        let mut bullet_indices_to_remove = Vec::new();

        for (index, bullet) in self.bullets.iter().enumerate() {
            let position = self.physics_engine.bodies[bullet.handle].translation();
            if position.x < 0.0 || position.x > bounds || position.y < 0.0 || position.y > bounds {
                bullet_indices_to_remove.push(index);
            }
        }

        bullet_indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for &index in &bullet_indices_to_remove {
            let bullet = &self.bullets[index];
            self.physics_engine.bodies.remove(
                bullet.handle,
                &mut self.physics_engine.islands,
                &mut self.physics_engine.colliders,
                &mut self.physics_engine.impulse_joints,
                &mut self.physics_engine.multibody_joints,
                true,
            );
            self.bullets.remove(index);
        }
    }

    pub fn reset_simulation(&mut self) {
        for entity in &mut self.entities {
            entity.score = 0;
        }

        // Delete all bullets
        for bullet in &self.bullets {
            self.physics_engine.bodies.remove(
                bullet.handle,
                &mut self.physics_engine.islands,
                &mut self.physics_engine.colliders,
                &mut self.physics_engine.impulse_joints,
                &mut self.physics_engine.multibody_joints,
                true,
            );
        }
        self.bullets.clear();
    }

    pub fn generate_map(&mut self) {
        let mut rng = rand::thread_rng();
        for entity in &mut self.entities {
            let random_x = rng.gen_range(10.0..1190.0);
            let random_y = rng.gen_range(10.0..990.0);
            let body = &mut self.physics_engine.bodies[entity.handle];
            body.set_translation(vector![random_x, random_y], true);
        }

        self.generate_obstacles();
    }

    pub fn add_ai(&mut self, name: String) {
        let entity = Entity::new(name, &mut self.physics_engine, true);
        self.entities.push(entity);
    }

    pub fn update_ai(&mut self) {
        let mut rng = rand::thread_rng();

        // Gather data first
        let updates: Vec<(RigidBodyHandle, Vector<f32>, Vector<f32>)> = self.entities.iter_mut()
            .filter_map(|entity| {
                if entity.is_ai {
                    // Randomly change the target position every few seconds
                    if entity.last_shot.elapsed().as_secs_f32() > rng.gen_range(1.0..3.0) {
                        entity.target_x = rng.gen_range(10.0..1190.0);
                        entity.target_y = rng.gen_range(10.0..990.0);
                        entity.last_shot = Instant::now();

                        // Change the gun orientation randomly at each target change
                        entity.gun_orientation = rng.gen_range(0.0..std::f64::consts::TAU);
                    }

                    // Move towards the target position
                    let current_pos = self.physics_engine.bodies[entity.handle].translation().clone();
                    let target_pos = vector![entity.target_x as f32, entity.target_y as f32];
                    let direction = target_pos - current_pos;
                    let distance = direction.norm();

                    if distance > 1.0 {
                        let movement = direction.normalize() * 1.0; // adjust the speed here
                        return Some((entity.handle, current_pos, movement));
                    }
                }
                None
            }).collect();

        // Apply updates
        for (handle, current_pos, movement) in updates {
            self.physics_engine.bodies[handle].set_next_kinematic_position(
                Isometry::translation(
                    current_pos.x + movement.x,
                    current_pos.y + movement.y,
                ),
            );
        }

        // Update entity positions and handle shooting
        for entity in &mut self.entities {
            if entity.is_ai {
                let current_pos = self.physics_engine.bodies[entity.handle].translation();
                let target_pos = vector![entity.target_x, entity.target_y];
                let direction = target_pos - current_pos;
                entity.self_orientation = direction.y.atan2(direction.x) as f64;


                // Randomly shoot a bullet every 500ms
                if entity.last_shot.elapsed().as_millis() >= 500 {
                    // Change the gun orientation randomly at each shoot
                    let random_angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let (sin, cos) = random_angle.sin_cos();

                    let bullet_handle = self.physics_engine.bodies.insert(
                        RigidBodyBuilder::dynamic()
                            .translation(*current_pos)
                            .linvel(vector![cos as f32 * 500.0, sin as f32 * 500.0])
                            .build(),
                    );
                    let bullet_collider = ColliderBuilder::ball(5.0)
                        .restitution(1.0)
                        .build();
                    self.physics_engine.colliders.insert_with_parent(bullet_collider, bullet_handle, &mut self.physics_engine.bodies);

                    let bullet = Bullet {
                        handle: bullet_handle,
                        shooter: entity.handle.clone(),
                    };

                    self.bullets.push(bullet);
                    entity.last_shot = Instant::now();
                }
            }
        }
    }
}

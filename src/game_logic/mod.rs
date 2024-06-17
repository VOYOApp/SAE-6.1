use rand::Rng;
use rapier2d::prelude::*;
use std::time::Instant;

use crate::bullet::bullet::Bullet;
use crate::entities::entity::Entity;
use crate::physics::physics::PhysicsEngine;

pub struct GameLogic {
    pub physics_engine: PhysicsEngine,
    pub entities: Vec<Entity>,
    pub bullets: Vec<Bullet>,
}

impl GameLogic {
    pub fn new() -> Self {
        let mut physics_engine = PhysicsEngine::default();

        // Create world boundaries
        let ground_handle = physics_engine.bodies.insert(RigidBodyBuilder::fixed().translation(vector![600.0, 0.0]).build());
        let ground_collider = ColliderBuilder::cuboid(600.0, 10.0).build();
        physics_engine.colliders.insert_with_parent(ground_collider, ground_handle, &mut physics_engine.bodies);

        let ceiling_handle = physics_engine.bodies.insert(RigidBodyBuilder::fixed().translation(vector![600.0, 1000.0]).build());
        let ceiling_collider = ColliderBuilder::cuboid(600.0, 10.0).build();
        physics_engine.colliders.insert_with_parent(ceiling_collider, ceiling_handle, &mut physics_engine.bodies);

        let left_wall_handle = physics_engine.bodies.insert(RigidBodyBuilder::fixed().translation(vector![0.0, 500.0]).build());
        let left_wall_collider = ColliderBuilder::cuboid(10.0, 500.0).build();
        physics_engine.colliders.insert_with_parent(left_wall_collider, left_wall_handle, &mut physics_engine.bodies);

        let right_wall_handle = physics_engine.bodies.insert(RigidBodyBuilder::fixed().translation(vector![1200.0, 500.0]).build());
        let right_wall_collider = ColliderBuilder::cuboid(10.0, 500.0).build();
        physics_engine.colliders.insert_with_parent(right_wall_collider, right_wall_handle, &mut physics_engine.bodies);

        Self {
            physics_engine,
            entities: Vec::new(),
            bullets: Vec::new(),
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

        let bullet_speed = 500.0;
        let bullet_direction = shooter.gun_orientation;
        let (sin, cos) = bullet_direction.sin_cos();
        let bullet_velocity = vector![bullet_speed * cos, bullet_speed * sin];

        let bullet_handle = self.physics_engine.bodies.insert(
            RigidBodyBuilder::dynamic()
                .translation(vector![shooter.x, shooter.y])
                // .linvel(bullet_velocity)
                .build(),
        );
        let bullet_collider = ColliderBuilder::ball(5.0)
            .restitution(0.0)
            .build();
        self.physics_engine.colliders.insert_with_parent(bullet_collider, bullet_handle, &mut self.physics_engine.bodies);

        let bullet = Bullet {
            handle: bullet_handle,
            shooter: shooter.handle.clone(),
        };

        self.bullets.push(bullet);
        self.entities[shooter_index].last_shot = Instant::now();
    }

    pub fn step(&mut self) {
        self.physics_engine.step();

        // Handle bullet collision with entities
        let mut bullet_indices_to_remove = Vec::new();
        for (bullet_index, bullet) in self.bullets.iter().enumerate() {
            let bullet_pos = self.physics_engine.bodies[bullet.handle].translation();

            for entity in &mut self.entities {
                if bullet.shooter != entity.handle {
                    let entity_pos = vector![entity.x, entity.y];
                    let distance = (bullet_pos - entity_pos).norm();
                    if distance < 15.0 {
                        entity.score += 1;
                        bullet_indices_to_remove.push(bullet_index);
                        break;
                    }
                }
            }
        }

        bullet_indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for &index in &bullet_indices_to_remove {
            self.bullets.remove(index);
        }
    }

    pub fn reset_simulation(&mut self) {
        for entity in &mut self.entities {
            entity.score = 0;
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

        // Update entity positions
        for entity in &mut self.entities {
            if entity.is_ai {
                let current_pos = self.physics_engine.bodies[entity.handle].translation();
                entity.x = current_pos.x;
                entity.y = current_pos.y;
            }
        }
    }
}

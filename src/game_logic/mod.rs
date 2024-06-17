use rand::Rng;
use rapier2d::prelude::*;

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

    pub fn remove_entity(&mut self, name: &str) {
        if let Some(pos) = self.entities.iter().position(|e| e.name == name) {
            let entity = self.entities.remove(pos);
            self.physics_engine.bodies.remove(entity.handle, &mut Default::default(), &mut Default::default(), &mut Default::default(), &mut Default::default(), false);
        }
    }

    pub fn shoot_ball(&mut self, entity_handle: RigidBodyHandle, line_thickness: f32) {
        if let Some(entity) = self.entities.iter().find(|e| e.handle == entity_handle) {
            let bullet = Bullet::new(entity.handle, &mut self.physics_engine, line_thickness);
            self.bullets.push(bullet);
        }
    }

    pub fn step(&mut self) {
        self.physics_engine.step();
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
        let ai_entity = Entity::new(name, &mut self.physics_engine, true);
        self.entities.push(ai_entity);
    }

    pub fn update_ai(&mut self) {
        let mut rng = rand::thread_rng();

        for entity in &mut self.entities {
            if entity.is_ai {
                // Generate a new random target position
                let target_x = rng.gen_range(10.0..1190.0);
                let target_y = rng.gen_range(10.0..990.0);

                // Get the current position
                let body = &mut self.physics_engine.bodies[entity.handle];
                let current_pos = body.translation();

                // Calculate the direction vector
                let direction = vector![
                    target_x - current_pos.x,
                    target_y - current_pos.y
                ];
                let direction = direction.normalize();

                // Set a velocity towards the target position
                let speed = 100.0; // You can adjust the speed as needed
                let velocity = direction * speed;
                body.set_linvel(velocity, true);
            }
        }
    }
}

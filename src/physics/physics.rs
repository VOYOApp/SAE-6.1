use std::time::{Duration, Instant};

// physics/mod.rs
use rapier2d::prelude::*;

use crate::app_defines::AppDefines;

/// Represents the physics engine and its components.
pub struct PhysicsEngine {
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub islands: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub ccd_solver: CCDSolver,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub query_pipeline: QueryPipeline,
    pub start_time: Instant,
    pub loop_duration: Duration,
    pub collision_events: Vec<CollisionEvent>,
}

impl Default for PhysicsEngine {
    /// Creates a new default `PhysicsEngine` instance.
    ///
    /// # Returns
    /// A new instance of `PhysicsEngine` with default settings.
    fn default() -> Self {
        Self {
            physics_pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            ccd_solver: CCDSolver::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            query_pipeline: QueryPipeline::new(),
            start_time: Instant::now(),
            loop_duration: Duration::new(5, 0),
            collision_events: Vec::new(),
        }
    }
}

impl PhysicsEngine {
    /// Advances the physics simulation by one step.
    ///
    /// Clears previous collision events and updates the physics world.
    pub fn step(&mut self) {
        self.collision_events.clear(); // Clear previous collision events
        let mut collision_event_handler = |event: &CollisionEvent| {
            self.collision_events.push(event.clone());
        };

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    /// Sets up the boundary colliders for the simulation area.
    pub fn setup_boundaries(&mut self) {
        let half_extents = vector![AppDefines::ARENA_WIDTH / 2.0, AppDefines::ARENA_HEIGHT / 2.0];
        let top_boundary = ColliderBuilder::cuboid(half_extents.x, 1.0)
            .translation(vector![half_extents.x, AppDefines::ARENA_HEIGHT])
            .build();
        let bottom_boundary = ColliderBuilder::cuboid(half_extents.x, 1.0)
            .translation(vector![half_extents.x, 0.0])
            .build();
        let left_boundary = ColliderBuilder::cuboid(1.0, half_extents.y)
            .translation(vector![0.0, half_extents.y])
            .build();
        let right_boundary = ColliderBuilder::cuboid(1.0, half_extents.y)
            .translation(vector![AppDefines::ARENA_WIDTH, half_extents.y])
            .build();

        self.colliders.insert(top_boundary);
        self.colliders.insert(bottom_boundary);
        self.colliders.insert(left_boundary);
        self.colliders.insert(right_boundary);
    }

    /// Sets up the physics simulation, including boundaries.
    pub fn setup_physics(&mut self) {
        self.setup_boundaries();
    }
}

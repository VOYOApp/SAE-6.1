use std::ops::{Div, Sub};

use rand::Rng;
use rapier2d::prelude::*;
use raylib::prelude::*;

const TARGET_FPS: u32 = 60;


struct Sprite {
    body_handle: RigidBodyHandle,
    color: Color,
}


fn configure_camera() -> Camera2D {
    Camera2D {
        target: Vector2::zero(),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    }
}

fn handle_camera_input(rl: &RaylibHandle, camera: &mut Camera2D) {

    let wheel = rl.get_mouse_wheel_move();
    if wheel != 0.0 {
        let mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), *camera);
        camera.offset = rl.get_mouse_position();
        camera.target = mouse_world_pos;

        let mut scale_factor = 1.0 + (0.25 * wheel.abs());
        if wheel < 0.0 {
            scale_factor = 1.0 / scale_factor;
        }
        camera.zoom *= scale_factor;
        camera.zoom = camera.zoom.clamp(0.125, 64.0);
    }
}

fn main() {
    // Initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(1200, 1000)
        .resizable()
        .title("Rapier2D with Raylib")
        .build();

    // Create the physics world
    let mut physics_pipeline = PhysicsPipeline::new();
    let gravity = vector![0.0, 0.0];  // No gravity
    let integration_parameters = IntegrationParameters::default();
    let mut islands = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let physics_hooks = ();
    let event_handler = ();

    // Define the world dimensions
    let world_width = 1200.0;
    let world_height = 1000.0;
    let thickness = 10.0;  // Thickness of the boundary walls

    let mut previous_mouse_position = rl.get_mouse_position();

    // Define the boundaries as static colliders
    let boundaries = [
        // Bottom boundary
        ColliderBuilder::cuboid(world_width / 2.0, thickness)
            .translation(vector![world_width / 2.0, thickness / 2.0])
            .build(),
        // Top boundary
        ColliderBuilder::cuboid(world_width / 2.0, thickness)
            .translation(vector![world_width / 2.0, world_height - thickness / 2.0])
            .build(),
        // Left boundary
        ColliderBuilder::cuboid(thickness, world_height / 2.0)
            .translation(vector![thickness / 2.0, world_height / 2.0])
            .build(),
        // Right boundary
        ColliderBuilder::cuboid(thickness, world_height / 2.0)
            .translation(vector![world_width - thickness / 2.0, world_height / 2.0])
            .build(),
    ];

    // Add the boundaries to the colliders set
    for boundary in boundaries.iter() {
        colliders.insert(boundary.clone());
    }

    // Create some random moving sprites
    let mut rng = rand::thread_rng();
    let mut sprites = Vec::new();

    for _ in 0..10 {
        let x = rng.gen_range(100.0..1100.0);
        let y = rng.gen_range(100.0..900.0);
        let vx = rng.gen_range(-100.0..100.0);
        let vy = rng.gen_range(-100.0..100.0);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .linvel(vector![vx, vy])
            .build();
        let collider = ColliderBuilder::ball(10.0).restitution(1.5).build();  // Add restitution to ensure bouncing

        let body_handle = bodies.insert(rigid_body);
        colliders.insert_with_parent(collider, body_handle, &mut bodies);

        sprites.push(Sprite {
            body_handle,
            color: Color::new(
                rng.gen_range(0..256) as u8,
                rng.gen_range(0..256) as u8,
                rng.gen_range(0..256) as u8,
                255,
            ),
        });
    }

    let mut camera = configure_camera();
    rl.set_target_fps(TARGET_FPS);


    // Main game loop
    while !rl.window_should_close() {
        handle_camera_input(&rl, &mut camera);

        let current_mouse_position = rl.get_mouse_position();

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let delta = Vector2::new(
                current_mouse_position.x - previous_mouse_position.x,
                current_mouse_position.y - previous_mouse_position.y,
            );
            let delta = Vector2::new(-delta.x / camera.zoom, -delta.y / camera.zoom);
            camera.target = Vector2::new(camera.target.x + delta.x, camera.target.y + delta.y);
        }
        previous_mouse_position = current_mouse_position;

        // Step the physics simulation
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut islands,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_mode2D(camera);

            // Draw boundaries
            d2.draw_rectangle(0, 0, 1200, 10, Color::GREEN); // Top
            d2.draw_rectangle(0, 990, 1200, 10, Color::GREEN); // Bottom
            d2.draw_rectangle(0, 0, 10, 1000, Color::GREEN); // Left
            d2.draw_rectangle(1190, 0, 10, 1000, Color::GREEN); // Right

            let rect = Rectangle::new(10.0, 10.0, 1180.0, 980.0);
            d2.gui_grid(rect, 50.0, 3);


            // Draw and update sprites
            for sprite in &sprites {
                if let Some(body) = bodies.get(sprite.body_handle) {
                    let pos = body.position().translation.vector;
                    d2.draw_circle(pos.x as i32, pos.y as i32, 10.0, sprite.color);
                }
            }
        }
        d.draw_fps(10,10)
    }
}

use std::ops::{Div, Sub};

use rapier2d::prelude::*;
use raylib::prelude::*;

const DOMINANT_COLOR: Color = Color::GREEN;
const LINE_THICKNESS: i32 = 5;
const WORLD_WIDTH: f32 = 1300.0;
const WORLD_HEIGHT: f32 = 1000.0;
const OBSTACLE_COUNT: i32 = 20;
const TARGET_FPS: u32 = 60;

fn main() {
    // Create the Rapier2D structures
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Create the world frame
    let world_collider_left = ColliderBuilder::cuboid(0.01, 50.0).build();
    let world_collider_right = ColliderBuilder::cuboid(0.01, 50.0).build();
    let world_collider_bottom = ColliderBuilder::cuboid(50.0, 0.01).build();
    let world_collider_top = ColliderBuilder::cuboid(50.0, 0.01).build();
    let world_collider_left_handle = collider_set.insert(world_collider_left.clone());
    let world_collider_right_handle = collider_set.insert(world_collider_right.clone());
    let world_collider_bottom_handle = collider_set.insert(world_collider_bottom.clone());
    let world_collider_top_handle = collider_set.insert(world_collider_top.clone());
    let world_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![-25.0, 0.0])
        .build();
    let world_body_handle = rigid_body_set.insert(world_body);
    collider_set.insert_with_parent(
        world_collider_left.clone(),
        world_body_handle,
        &mut rigid_body_set,
    );
    let world_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![25.0, 0.0])
        .build();
    let world_body_handle = rigid_body_set.insert(world_body);
    collider_set.insert_with_parent(
        world_collider_right.clone(),
        world_body_handle,
        &mut rigid_body_set,
    );
    let world_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, -25.0])
        .build();
    let world_body_handle = rigid_body_set.insert(world_body);
    collider_set.insert_with_parent(
        world_collider_bottom.clone(),
        world_body_handle,
        &mut rigid_body_set,
    );
    let world_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![0.0, 25.0])
        .build();
    let world_body_handle = rigid_body_set.insert(world_body);
    collider_set.insert_with_parent(
        world_collider_top.clone(),
        world_body_handle,
        &mut rigid_body_set,
    );

    // Create the obstacles
    for i in 0..OBSTACLE_COUNT {
        let obstacle_collider = ColliderBuilder::cuboid(1.0, 1.0).build();
        let obstacle_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![i as f32 * 2.0 - 10.0, 10.0])
            .build();
        let obstacle_body_handle = rigid_body_set.insert(obstacle_body);
        collider_set.insert_with_parent(
            obstacle_collider,
            obstacle_body_handle,
            &mut rigid_body_set,
        );
    }

    // Create the bouncing ball
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 0.0])
        .build();
    let ball_collider = ColliderBuilder::ball(0.5).restitution(2.5).build();
    let ball_body_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

    // Physics structures
    let gravity = vector![0.10, -0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase: DefaultBroadPhase = Default::default();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();
    // Initialization
    let (mut rl, thread) = raylib::init()
        .size(1200, 1000)
        .resizable()
        .title("raylib [core] example - 2d camera mouse zoom")
        .build();

    let mut camera = Camera2D {
        target: Vector2::zero(),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(TARGET_FPS);


    // Main game loop
    while !rl.window_should_close() {
        // Camera translation based on mouse right click
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let delta = rl.get_mouse_position().sub(camera.offset);
            camera.target = camera.target.sub(delta.div(camera.zoom));
            camera.offset = rl.get_mouse_position();
        }

        // Zoom based on mouse wheel
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            // Get the world point that is under the mouse
            let mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), &camera);

            // Set the offset to where the mouse is
            camera.offset = rl.get_mouse_position();

            // Set the target to match
            camera.target = mouse_world_pos;

            // Zoom increment
            let mut scale_factor = 1.0 + (0.25 * wheel.abs());
            if wheel < 0.0 {
                scale_factor = 1.0 / scale_factor;
            }
            camera.zoom = camera.zoom * scale_factor;
            camera.zoom = camera.zoom.clamp(0.125, 64.0);
        }


        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Step the physics simulation
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        // Get the ball's current position
        let ball_body = &rigid_body_set[ball_body_handle];
        let ball_position = ball_body.translation();


        {
            let mut d2 = d.begin_mode2D(camera);

            let mut rect = Rectangle::new(-850.0, 300.0, 1250.0, 1250.0);
            // Draw the world frame in green
            d2.gui_grid(rect, 20.0, 3);

            d2.draw_rectangle_rec(rect, Color::new(0, 0, 0, 0));
            d2.draw_rectangle_lines_ex(rect, LINE_THICKNESS, DOMINANT_COLOR);


            // Draw ball
            d2.draw_circle(
                (ball_position.x * 50.0 + 400.0) as i32,
                (300.0 - ball_position.y * 50.0) as i32,
                25.0,
                Color::MAROON,
            );
        }

        d.draw_fps(10, 10);
    }
}

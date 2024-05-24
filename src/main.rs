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

    // Create the ground
    let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).build();
    collider_set.insert(ground_collider);

    // Create the bouncing ball
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .build();
    let ball_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

    // Physics structures
    let gravity = vector![0.0, -9.81];
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
    let rect = Rectangle::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT);


    rl.set_target_fps(TARGET_FPS);

    // TODO: Generate random obstacles to be added into the scene here


    // Main game loop
    while !rl.window_should_close() {
        // Camera translation based on mouse right click
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
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



        // Display ball altitude in console
        println!("Ball altitude: {}", ball_position.y);

        {
            let mut d2 = d.begin_mode2D(camera);

            // Draw the grid
            d2.gui_grid(
                Rectangle::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
                50.0,
                5.0 as i32,
            );

            // Draw a transparent rectangle with green borders
            d2.draw_rectangle_rec(rect, Color::new(0, 0, 0, 0));
            d2.draw_rectangle_lines_ex(rect, LINE_THICKNESS, DOMINANT_COLOR);

            // Draw ground
            d2.draw_rectangle(0, 300, 800, 10, Color::DARKGRAY);

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

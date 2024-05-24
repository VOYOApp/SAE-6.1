use rapier2d::prelude::*;
use raylib::prelude::*;


const DOMINANT_COLOR: Color = Color::GREEN;
const LINE_THICKNESS: i32 = 5;
const WORLD_WIDTH: f32 = 1300.0;
const WORLD_HEIGHT: f32 = 1000.0;
const OBSTACLE_COUNT: i32 = 20;
const TARGET_FPS: i32 = 60;
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

    // Initialize Raylib
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

    let mut zoom_mode = 0; // 0-Mouse Wheel, 1-Mouse Move

    rl.set_target_fps(60);

    // Main game loop
    while !rl.window_should_close() {
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

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Draw ground
        d.draw_rectangle(0, 300, 800, 10, Color::DARKGRAY);

        // Draw ball
        d.draw_circle(
            (ball_position.x * 50.0 + 400.0) as i32,
            (300.0 - ball_position.y * 50.0) as i32,
            25.0,
            Color::MAROON,
        );

        // Display ball altitude in console
        println!("Ball altitude: {}", ball_position.y);
    }
}

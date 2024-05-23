use std::ops::{Div, Sub};

use rand::Rng;
use raylib::prelude::*;

const DOMINANT_COLOR: Color = Color::GREEN;
const LINE_THICKNESS: i32 = 5;
const WORLD_WIDTH: f32 = 1300.0;
const WORLD_HEIGHT: f32 = 1000.0;
const OBSTACLE_COUNT: i32 = 20;

fn main() {
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

    let mut zoom_mode = 0; // 0-Mouse Wheel, 1-Mouse Move

    // Bouncing ball variables
    let mut ball_position = Vector2::new(500.0, 500.0); // Initial position within the rectangle
    let mut ball_speed = Vector2::new(5.0, 4.0);
    let ball_radius = 20;
    let rect = Rectangle::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT);
    let mut pause = false;


    // Generate random obstacles
    let mut rng = rand::thread_rng();
    let mut obstacles = Vec::new();
    for _ in 0..OBSTACLE_COUNT {
        let size = rng.gen_range(50.0..100.0);
        let x = rng.gen_range(0.0..(rect.width - size));
        let y = rng.gen_range(0.0..(rect.height - size));
        obstacles.push(Rectangle::new(x, y, size, size));
    }

    rl.set_target_fps(60);

    // Main game loop
    while !rl.window_should_close() {
        // Update
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            zoom_mode = 0;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            zoom_mode = 1;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }

        if !pause {
            // Ball movement
            ball_position.x += ball_speed.x;
            ball_position.y += ball_speed.y;

            // Check collision with rectangle boundaries
            if (ball_position.x >= (rect.x + rect.width - ball_radius as f32)) || (ball_position.x <= (rect.x + ball_radius as f32)) {
                ball_speed.x *= -1.0;
            }
            if (ball_position.y >= (rect.y + rect.height - ball_radius as f32)) || (ball_position.y <= (rect.y + ball_radius as f32)) {
                ball_speed.y *= -1.0;
            }

            // Check collision with obstacles
            for obstacle in &obstacles {
                if check_collision_circle_rec(ball_position, ball_radius as f32, *obstacle) {
                    // Reflect ball direction
                    if ball_position.x + ball_radius as f32 >= obstacle.x && ball_position.x - ball_radius as f32 <= obstacle.x + obstacle.width {
                        ball_speed.y *= -1.0;
                    }
                    if ball_position.y + ball_radius as f32 >= obstacle.y && ball_position.y - ball_radius as f32 <= obstacle.y + obstacle.height {
                        ball_speed.x *= -1.0;
                    }
                }
            }
        }

        // Camera translation based on mouse right click
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let delta = rl.get_mouse_position().sub(camera.offset);
            camera.target = camera.target.sub(delta.div(camera.zoom));
            camera.offset = rl.get_mouse_position();
        }

        // Camera zoom
        if zoom_mode == 0 {
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
        } else {
            // Zoom based on mouse left click
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                // Get the world point that is under the mouse
                let mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), &camera);

                // Set the offset to where the mouse is
                camera.offset = rl.get_mouse_position();

                // Set the target to match
                camera.target = mouse_world_pos;
            }
            if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                // Zoom increment
                let delta_x = rl.get_mouse_x();
                let mut scale_factor = 1.0 + (0.01 * delta_x.abs() as f32);
                if delta_x < 0.0 as i32 {
                    scale_factor = 1.0 / scale_factor;
                }
                camera.zoom = camera.zoom * scale_factor;
                camera.zoom = camera.zoom.clamp(0.125, 64.0);
            }
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_mode2D(camera);

            // Draw the grid
            d2.gui_grid(
                Rectangle::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
                50.0,
                5.0 as i32,
            );

            // Draw a transparent rectangle with green borders
            d2.draw_rectangle_rec(rect, Color::new(0, 0, 0, 0)); // Transparent fill
            d2.draw_rectangle_lines_ex(rect, LINE_THICKNESS, DOMINANT_COLOR); // Thickness of 5

            // Draw the obstacles
            for obstacle in &obstacles {
                d2.draw_rectangle_rec(*obstacle, Color::new(0, 0, 0, 0));
                let mut thickness = LINE_THICKNESS - 1;
                if thickness < 1 {
                    thickness = 1;
                }
                d2.draw_rectangle_lines_ex(*obstacle, thickness, DOMINANT_COLOR);
            }

            // Draw the bouncing ball
            d2.draw_circle_v(ball_position, ball_radius as f32, Color::MAROON);
        }

        d.draw_fps(10, 10);
    }
}

// Collision detection function
fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    let rec_center = Vector2::new(rec.x + rec.width / 2.0, rec.y + rec.height / 2.0);
    let dx = (center.x - rec_center.x).abs().min(rec.width / 2.0);
    let dy = (center.y - rec_center.y).abs().min(rec.height / 2.0);
    let closest = Vector2::new(rec_center.x + dx * (if center.x < rec_center.x { -1.0 } else { 1.0 }),
                               rec_center.y + dy * (if center.y < rec_center.y { -1.0 } else { 1.0 }));
    center.distance_to(closest) < radius
}

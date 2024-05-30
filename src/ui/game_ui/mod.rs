// main.rs

#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)]

use std::time::{Duration, Instant};

use eframe::egui;
use egui::Vec2b;
use egui_extras::*;
use egui_plot::*;
use egui_plot::Line;
use rand::Rng;
use rapier2d::prelude::*;

pub(crate) struct GameUI {
    physics_pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    ccd_solver: CCDSolver,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    query_pipeline: QueryPipeline,
    balls: Vec<RigidBodyHandle>,
    start_time: Instant,
    loop_duration: Duration,
}

impl Default for GameUI {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let gravity = vector![0.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let islands = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let ccd_solver = CCDSolver::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let query_pipeline = QueryPipeline::new();

        // Create world boundaries
        let ground_handle = bodies.insert(RigidBodyBuilder::fixed().translation(vector![600.0, 0.0]).build());
        let ground_collider = ColliderBuilder::cuboid(600.0, 10.0).build();
        colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

        let ceiling_handle = bodies.insert(RigidBodyBuilder::fixed().translation(vector![600.0, 1000.0]).build());
        let ceiling_collider = ColliderBuilder::cuboid(600.0, 10.0).build();
        colliders.insert_with_parent(ceiling_collider, ceiling_handle, &mut bodies);

        let left_wall_handle = bodies.insert(RigidBodyBuilder::fixed().translation(vector![0.0, 500.0]).build());
        let left_wall_collider = ColliderBuilder::cuboid(10.0, 500.0).build();
        colliders.insert_with_parent(left_wall_collider, left_wall_handle, &mut bodies);

        let right_wall_handle = bodies.insert(RigidBodyBuilder::fixed().translation(vector![1200.0, 500.0]).build());
        let right_wall_collider = ColliderBuilder::cuboid(10.0, 500.0).build();
        colliders.insert_with_parent(right_wall_collider, right_wall_handle, &mut bodies);

        // Create balls with random initial positions and velocities
        let mut balls = Vec::new();
        for _ in 0..2000 {
            let x = rng.gen_range(50.0..1150.0);
            let y = rng.gen_range(50.0..950.0);
            let vx = rng.gen_range(-50.0..50.0);
            let vy = rng.gen_range(-50.0..50.0);
            let ball_handle = bodies.insert(
                RigidBodyBuilder::dynamic()
                    .translation(vector![x, y])
                    .linvel(vector![vx, vy])
                    .build(),
            );
            let ball_collider = ColliderBuilder::ball(10.0).restitution(1.0).build();
            colliders.insert_with_parent(ball_collider, ball_handle, &mut bodies);
            balls.push(ball_handle);
        }

        // Create diamond-shaped obstacles
        let diamond_handle = bodies.insert(RigidBodyBuilder::fixed().translation(vector![600.0, 500.0]).build());
        let diamond_collider = ColliderBuilder::cuboid(50.0, 50.0).rotation(0.785398).build();
        colliders.insert_with_parent(diamond_collider, diamond_handle, &mut bodies);


        Self {
            physics_pipeline,
            gravity,
            integration_parameters,
            islands,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            ccd_solver,
            impulse_joints,
            multibody_joints,
            query_pipeline,
            balls,
            start_time: Instant::now(),
            loop_duration: Duration::new(5, 0),
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the physics
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

        let points: Vec<_> = self.balls.iter().map(|&handle| {
            let ball = &self.bodies[handle];
            [ball.translation().x as f64, ball.translation().y as f64]
        }).collect();

        // Get window width
        let mut window_width = ctx.screen_rect().width();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("left_panel")
                .max_width(window_width - 350.0)
                .min_width(window_width - 350.0)
                .show_inside(ui, |ui| {

                // Plot view with dynamic points
                let plot_points = Points::new(points)
                    .color(egui::Color32::from_rgb(255, 0, 0))
                    .name("Balls");

                Plot::new("dynamic_plot")
                    .show_axes([false, false])
                    .show_grid([false, false])
                    .allow_boxed_zoom(false)
                    .auto_bounds(Vec2b::new(false, false))
                    .data_aspect(1.0)
                    .show(ui, |plot_ui| {
                        plot_ui.points(plot_points);

                        // World Boundaries
                        let world_boundary = Line::new(PlotPoints::new(vec![[0.0, 0.0], [1200.0, 0.0], [1200.0, 1000.0], [0.0, 1000.0], [0.0, 0.0]]))
                            .color(egui::Color32::GREEN)
                            .name("World Boundary")
                            .width(4.0)
                            .style(LineStyle::Solid);
                        plot_ui.line(world_boundary);

                        // Diamond shaped obstacles
                        let diamond_line = Line::new(PlotPoints::new(vec![[600.0, 500.0], [650.0, 550.0], [600.0, 600.0], [550.0, 550.0], [600.0, 500.0]]))
                            .color(egui::Color32::LIGHT_BLUE)
                            .name("Diamond")
                            .width(4.0)
                            .style(LineStyle::Solid);
                        plot_ui.line(diamond_line);
                    });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                TableBuilder::new(ui)
                    .column(Column::exact(200.0).resizable(false))
                    .column(Column::exact(100.0).resizable(false))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Player Name");
                        });
                        header.col(|ui| {
                            ui.heading("Score");
                        });
                    })
                    .body(|mut body| {
                        let players = vec!["Lopi5555", "Hethan_hdb", "Vavaaaaaah"];
                        let scores = vec!["5", "19", "0"];
                        let padding = 10.0; // Define the left padding

                        for (index, (player, score)) in players.iter().zip(scores.iter()).enumerate() {
                            body.row(30.0, |mut row| {
                                let bg_color = if index % 2 == 0 {
                                    egui::Color32::from_gray(20) // Light gray for even rows
                                } else {
                                    egui::Color32::from_gray(24) // Almost white for odd rows
                                };
                                row.col(|ui| {
                                    ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                    ui.horizontal_centered(|ui| {
                                        ui.add_space(padding); // Add left padding
                                        ui.colored_label(egui::Color32::from_rgb(255, 255, 255), *player);
                                    });
                                });
                                row.col(|ui| {
                                    ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                    ui.horizontal_centered(|ui| {
                                        ui.add_space(padding); // Add left padding
                                        ui.colored_label(egui::Color32::from_rgb(255, 255, 255), *score);
                                    });
                                });
                            });
                        }
                    });
            });
        });

        // Request a repaint to animate the plot
        ctx.request_repaint();
    }
}
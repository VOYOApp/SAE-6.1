use std::time::Duration;
use std::time::Instant;
use rand::Rng;

use eframe::egui;
use egui::{Align2, Context, TopBottomPanel};
use egui_extras::*;
use egui_plot::*;
use rapier2d::prelude::*;

use crate::ball::ball::Ball;
use crate::physics::physics::PhysicsEngine;

pub struct GameUI {
    physics_engine: PhysicsEngine,
    balls: Vec<Ball>,
    entities: Vec<Entity>,
    line_thickness: f32,
    show_names: bool,
    show_background: bool,
    last_shot: Instant,
}

struct Entity {
    name: String,
    score: i32,
    handle: RigidBodyHandle,
}

impl GameUI {
    fn display_entities(&self, plot_ui: &mut PlotUi) {
        for entity in &self.entities {
            let body = &self.physics_engine.bodies[entity.handle];
            let pos = [body.translation().x as f64, body.translation().y as f64];
            let angle = body.rotation().angle();

            let points = vec![
                [
                    pos[0] + self.line_thickness as f64 * 1.5 * angle.cos() as f64,
                    pos[1] + self.line_thickness as f64 * 1.5 * angle.sin() as f64,
                ],
                [
                    pos[0] - self.line_thickness as f64 * 1.5 * (angle as f64 + std::f64::consts::FRAC_PI_4).cos(),
                    pos[1] - self.line_thickness as f64 * 1.5 * (angle as f64 + std::f64::consts::FRAC_PI_4).sin(),
                ],
                [
                    pos[0] - self.line_thickness as f64 * 1.5 * (angle as f64 - std::f64::consts::FRAC_PI_4).cos(),
                    pos[1] - self.line_thickness as f64 * 1.5 * (angle as f64 - std::f64::consts::FRAC_PI_4).sin(),
                ],
            ];

            plot_ui.line(
                Line::new(PlotPoints::new(points))
                    .color(egui::Color32::LIGHT_BLUE)
                    .width(self.line_thickness),
            );

            if self.show_names {
                let pos_with_offset = [pos[0], pos[1] + 20.0]; // Add 20 to the y-coordinate
                plot_ui.text(
                    Text::new(PlotPoint::from(pos_with_offset), &entity.name)
                        .color(egui::Color32::WHITE)
                        .anchor(Align2::CENTER_CENTER),
                );
            }
        }
    }

    fn shoot_ball(&mut self, entity_handle: RigidBodyHandle) {
        if let Some(entity) = self.entities.iter().find(|e| e.handle == entity_handle) {
            let body = &self.physics_engine.bodies[entity.handle];
            let pos = body.translation();
            let angle = body.rotation().angle();
            let direction = vector![angle.cos(), angle.sin()];

            let ball_handle = self.physics_engine.bodies.insert(
                RigidBodyBuilder::dynamic()
                    .translation(*pos)
                    .linvel(direction * 500.0)
                    .build(),
            );
            let ball_collider = ColliderBuilder::ball(self.line_thickness).build();
            self.physics_engine.colliders.insert_with_parent(
                ball_collider,
                ball_handle,
                &mut self.physics_engine.bodies,
            );

            self.balls.push(Ball { handle: ball_handle, shooter: entity.handle });
        }
    }

    fn add_entity(&mut self, name: String) {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(10.0..1190.0);
        let random_y = rng.gen_range(10.0..990.0);

        let player_name = format!("Player {}", self.entities.len() + 1);
        let handle = self.physics_engine.bodies.insert(
            RigidBodyBuilder::kinematic_position_based()
                .translation(vector![random_x, random_y])
                .build(),
        );
        let collider = ColliderBuilder::cuboid(10.0, 10.0)
            .restitution(0.0) // No bouncing
            .build();
        self.physics_engine.colliders.insert_with_parent(
            collider,
            handle,
            &mut self.physics_engine.bodies,
        );

        self.entities.push(Entity {
            name: player_name,
            score: 0,
            handle,
        });
    }

    fn remove_entity(&mut self, name: &str) {
        if let Some(pos) = self.entities.iter().position(|e| e.name == name) {
            let entity = self.entities.remove(pos);
            self.physics_engine.bodies.remove(entity.handle, &mut Default::default(), &mut Default::default(), &mut Default::default(), &mut Default::default(), false);
        }
    }

    fn reset_simulation(&mut self) {
        for entity in &mut self.entities {
            entity.score = 0;
        }
        self.balls.clear();
    }

    fn generate_map(&mut self) {
        let mut rng = rand::thread_rng();
        for entity in &mut self.entities {
            let random_x = rng.gen_range(10.0..1190.0);
            let random_y = rng.gen_range(10.0..990.0);
            let body = &mut self.physics_engine.bodies[entity.handle];
            body.set_translation(vector![random_x, random_y], true);
        }
    }

    fn show_menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Toggle Names").clicked() {
                    self.show_names = !self.show_names;
                }
                if ui.button("T+").clicked() {
                    if self.line_thickness < 20.0 {
                        self.line_thickness += 1.0;
                    }
                }
                if ui.button("T-").clicked() {
                    if self.line_thickness > 1.0 {
                        self.line_thickness -= 1.0;
                    }
                }
                if ui.button("Reset Simulation").clicked() {
                    self.reset_simulation();
                }
                if ui.button("Generate Map").clicked() {
                    self.generate_map();
                }
                if ui.button("Show Background").clicked() {
                    self.show_background = !self.show_background;
                }
                if ui.button("Add Entity").clicked() {
                    self.add_entity("Player".to_string());
                }
                if ui.button("Remove Entity").clicked() {
                    self.remove_entity("Player");
                }
            });
        });
    }
}

impl Default for GameUI {
    fn default() -> Self {
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
            balls: Vec::new(),
            entities: Vec::new(),
            line_thickness: 4.0,
            show_names: true,
            show_background: true,
            last_shot: Instant::now(),
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu(ctx);

        // Shoot balls every 500ms
        if self.last_shot.elapsed() > Duration::from_millis(500) {
            if let Some(entity) = self.entities.first() {
                self.shoot_ball(entity.handle);
            }
            self.last_shot = Instant::now();
        }

        // Update the physics
        self.physics_engine.step();

        // // Check for ball collisions and remove collided balls
        // let mut collision_events = vec![];
        // self.physics_engine.events.collect(&mut collision_events);
        //
        // for event in collision_events {
        //     if let CollisionEvent::Started(collider1, collider2, _) = event {
        //         if let Some(ball_index) = self.balls.iter().position(|ball| {
        //             ball.handle == self.physics_engine.colliders[collider1].parent() || ball.handle == self.physics_engine.colliders[collider2].parent()
        //         }) {
        //             self.balls.remove(ball_index);
        //         }
        //     }
        // }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("entity_list").show_inside(ui, |ui| {
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
                        let padding = 10.0;

                        for (index, entity) in self.entities.iter().enumerate() {
                            body.row(30.0, |mut row| {
                                let bg_color = if index % 2 == 0 {
                                    egui::Color32::from_gray(20)
                                } else {
                                    egui::Color32::from_gray(24)
                                };
                                row.col(|ui| {
                                    ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                    ui.horizontal_centered(|ui| {
                                        ui.add_space(padding);
                                        ui.colored_label(egui::Color32::from_rgb(255, 255, 255), &entity.name);
                                    });
                                });
                                row.col(|ui| {
                                    ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                    ui.horizontal_centered(|ui| {
                                        ui.add_space(padding);
                                        ui.colored_label(egui::Color32::from_rgb(255, 255, 255), &entity.score.to_string());
                                    });
                                });
                            });
                        }
                    });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                Plot::new("dynamic_plot")
                    .show_axes([false, false])
                    .allow_boxed_zoom(false)
                    .show_grid(false)
                    .show_x(false)
                    .show_y(false)
                    .data_aspect(1.0)
                    .show(ui, |plot_ui| {
                        if self.show_background {
                            let x_lines: Vec<f64> = (0..=1200).step_by(50).map(|x| x as f64).collect();
                            let y_lines: Vec<f64> = (0..=1000).step_by(50).map(|y| y as f64).collect();

                            for &x in &x_lines {
                                let vertical_line = Line::new(PlotPoints::new(vec![[x, 0.0], [x, 1000.0]]))
                                    .color(egui::Color32::from_rgb(0, 40, 0))
                                    .width(self.line_thickness / 3.0)
                                    .style(LineStyle::Solid);
                                plot_ui.line(vertical_line);
                            }

                            for &y in &y_lines {
                                let horizontal_line = Line::new(PlotPoints::new(vec![[0.0, y], [1200.0, y]]))
                                    .color(egui::Color32::from_rgb(0, 40, 0))
                                    .width(self.line_thickness / 3.0)
                                    .style(LineStyle::Solid);
                                plot_ui.line(horizontal_line);
                            }
                        }

                        let plot_points = Points::new(
                            self.balls
                                .iter()
                                .map(|ball| {
                                    let pos = self.physics_engine.bodies[ball.handle].translation();
                                    [pos.x as f64, pos.y as f64]
                                })
                                .collect::<Vec<_>>(),
                        )
                            .radius(self.line_thickness / 2.0)  // Adjust radius based on line_thickness
                            .name("Balls");
                        plot_ui.points(plot_points);

                        self.display_entities(plot_ui); // Display entities

                        let world_boundary = Line::new(PlotPoints::new(vec![
                            [0.0, 0.0],
                            [1200.0, 0.0],
                            [1200.0, 1000.0],
                            [0.0, 1000.0],
                            [0.0, 0.0],
                        ]))
                            .color(egui::Color32::GREEN)
                            .name("World Boundary")
                            .width(self.line_thickness)
                            .style(LineStyle::Solid);
                        plot_ui.line(world_boundary);
                    });
            });

            ctx.request_repaint();
        });
    }
}

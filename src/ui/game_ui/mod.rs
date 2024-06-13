use eframe::egui;
use egui::{Context, TopBottomPanel};
use egui_extras::*;
use egui_plot::*;
use rapier2d::prelude::*;

use crate::ball::ball::{Ball, create_balls};
use crate::physics::physics::PhysicsEngine;

pub struct GameUI {
    physics_engine: PhysicsEngine,
    balls: Vec<Ball>,
    entities: Vec<Entity>,
    line_thickness: f32,
    show_names: bool,
    show_background: bool,
}


struct Entity {
    name: String,
    score: i32,
    handle: RigidBodyHandle,
}


impl GameUI {
    fn add_entity(&mut self, name: String) {
        let handle = self.physics_engine.bodies.insert(
            RigidBodyBuilder::dynamic()
                .translation(vector![600.0, 500.0])
                .build(),
        );
        let collider = ColliderBuilder::cuboid(10.0, 10.0).build();
        self.physics_engine.colliders.insert_with_parent(
            collider,
            handle,
            &mut self.physics_engine.bodies,
        );

        self.entities.push(Entity {
            name,
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

    fn display_entities(&self, plot_ui: &mut PlotUi) {
        for entity in &self.entities {
            let body = &self.physics_engine.bodies[entity.handle];
            let pos = [body.translation().x as f64, body.translation().y as f64];
            plot_ui.points(Points::new(vec![pos]).color(egui::Color32::YELLOW).name("Entity"));
        }
    }

    fn show_menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Names").clicked() {}
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
                    if self.line_thickness < 20.0 {
                        self.line_thickness += 1.0;
                    }
                }
                if ui.button("Generate Map").clicked() {
                    if self.line_thickness < 20.0 {
                        self.line_thickness += 1.0;
                    }
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

        let balls = create_balls(&mut physics_engine, 200);

        Self {
            physics_engine,
            balls,
            entities: Vec::new(),
            line_thickness: 4.0,
            show_names: true,
            show_background: true,
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu(ctx);

        // Update the physics
        self.physics_engine.step();

        let points: Vec<_> = self.balls.iter().map(|ball| {
            let ball = &self.physics_engine.bodies[ball.handle];
            [ball.translation().x as f64, ball.translation().y as f64]
        }).collect();

        let window_width = ctx.screen_rect().width();

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
        });

        ctx.request_repaint();
    }
}

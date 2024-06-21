use eframe::egui;
use egui::{Align2, Context, TopBottomPanel};
use egui_extras::*;
use egui_plot::*;

use crate::game_logic::GameLogic;

/// Represents the user interface for the game.
pub struct GameUI {
    game_logic: GameLogic,
    line_thickness: f32,
    show_names: bool,
    show_background: bool,
}

impl GameUI {
    /// Draws the obstacles on the plot.
    ///
    /// # Parameters
    /// - `plot_ui`: The `PlotUi` instance where obstacles will be drawn.
    fn draw_obstacles(&self, plot_ui: &mut PlotUi) {
        for obstacle in &self.game_logic.obstacles {
            let position = obstacle.position;
            let line_thickness = self.line_thickness / 2.0;

            let diamond_points = vec![
                [position.0, position.1 - 10.0],
                [position.0 - 10.0, position.1],
                [position.0, position.1 + 10.0],
                [position.0 + 10.0, position.1],
                [position.0, position.1 - 10.0], // close the diamond shape
            ];

            plot_ui.line(
                Line::new(PlotPoints::new(diamond_points))
                    .color(egui::Color32::GREEN)
                    .width(line_thickness),
            );
        }
    }

    /// Displays the entities on the plot.
    ///
    /// # Parameters
    /// - `plot_ui`: The `PlotUi` instance where entities will be displayed.
    fn display_entities(&self, plot_ui: &mut PlotUi) {
        for entity in &self.game_logic.entities {
            let body = &self.game_logic.physics_engine.bodies[entity.handle];
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

    /// Displays the menu bar at the top of the UI.
    ///
    /// # Parameters
    /// - `ctx`: The `Context` instance for rendering the menu.
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
                    self.game_logic.reset_simulation();
                }
                if ui.button("Generate Map").clicked() {
                    self.game_logic.generate_map();
                }
                if ui.button("Show Background").clicked() {
                    self.show_background = !self.show_background;
                }
                if ui.button("Add Entity").clicked() {
                    self.game_logic.add_entity("Player".to_string());
                }
                if ui.button("Add AI").clicked() { // New AI button
                    self.game_logic.add_ai("AI Bot".to_string());
                }
            });
        });
    }
}

impl Default for GameUI {
    /// Creates a new default `GameUI` instance.
    ///
    /// # Returns
    /// A new instance of `GameUI` with default settings.
    fn default() -> Self {
        let mut game_logic = GameLogic::new();
        game_logic.generate_map();

        Self {
            game_logic,
            line_thickness: 4.0,
            show_names: true,
            show_background: true,
        }
    }
}

impl eframe::App for GameUI {
    /// Updates the game UI.
    ///
    /// # Parameters
    /// - `ctx`: The `Context` instance for rendering.
    /// - `_frame`: The `Frame` instance for the application window.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu(ctx);

        // Update AI movement
        self.game_logic.update_ai();

        // Update the physics
        self.game_logic.step();

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

                        for (index, entity) in self.game_logic.entities.iter().enumerate() {
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
                            self.game_logic.bullets
                                .iter()
                                .map(|bullet| {
                                    let pos = self.game_logic.physics_engine.bodies[bullet.handle].translation();
                                    [pos.x as f64, pos.y as f64]
                                })
                                .collect::<Vec<_>>(),
                        )
                            .radius(self.line_thickness / 2.0)
                            .name("Bullets");
                        plot_ui.points(plot_points);

                        self.display_entities(plot_ui);

                        self.draw_obstacles(plot_ui);

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

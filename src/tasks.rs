mod types;
use chrono::prelude::*;
use egui::{Align::*, Color32, Layout, RichText, Ui};
use types::Task;

use crate::utils::format_datetime;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Tasks {
    pub tasks: Vec<Task>,

    #[serde(skip)]
    task_text_entry: String,
    #[serde(skip)]
    deleting_task_idx: Option<usize>,
}

impl Tasks {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn delete_task_confirmation(&mut self, ui: &mut Ui, task_idx: usize) {
        ui.with_layout(Layout::top_down(Center), |ui| {
            ui.heading("Are you sure?");
            ui.label(format!("About to delete \"{}\"", self.tasks[task_idx].text));
        });
        ui.with_layout(Layout::bottom_up(Center), |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(
                        RichText::new("Delete")
                            .background_color(Color32::DARK_RED)
                            .color(Color32::WHITE),
                    )
                    .clicked()
                {
                    self.tasks.remove(task_idx);
                    self.deleting_task_idx = None;
                }
                if ui.button(RichText::new("Cancel")).clicked() {
                    self.deleting_task_idx = None;
                }
            });
        });
    }

    fn show_task(&mut self, ui: &mut Ui, task_idx: usize) {
        let task = &mut self.tasks[task_idx];
        ui.horizontal(|ui| {
            if ui.checkbox(&mut task.is_done, &task.text).changed() && task.is_done {
                task.done_at = Some(Local::now());
            }
            if ui.small_button("🗑").clicked() && self.deleting_task_idx.is_none() {
                self.deleting_task_idx = Some(task_idx);
            };

            ui.with_layout(Layout::right_to_left(Center), |ui| {
                ui.small(format!("created {}", format_datetime(task.created_at)));
                ui.add_space(1.);
                if let Some(timestamp) = task.done_at {
                    ui.small(format!("done {}", format_datetime(timestamp)));
                }
            });
        });
    }
}

impl eframe::App for Tasks {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");
            ui.horizontal_top(|ui| {
                if (ui
                    .text_edit_singleline(&mut self.task_text_entry)
                    .lost_focus()
                    && ui.input().key_pressed(egui::Key::Enter))
                    || ui.button("Create task").clicked()
                {
                    self.tasks.push(Task::new(self.task_text_entry.clone()));
                    self.task_text_entry.clear();
                }
            });
            for task_idx in 0..self.tasks.len() {
                ui.separator();
                self.show_task(ui, task_idx);
            }
            ui.separator();
            egui::warn_if_debug_build(ui);
        });
        if let Some(task_idx) = self.deleting_task_idx {
            egui::Window::new("Delete task")
                .default_width(100.)
                .default_height(200.)
                .show(ctx, |ui| {
                    self.delete_task_confirmation(ui, task_idx);
                });
        }
    }
}

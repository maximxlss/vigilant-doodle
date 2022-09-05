mod types;

use chrono::prelude::*;
use egui::{Align::*, Color32, Layout, RichText, Ui, TextEdit};
use types::Note;

use crate::utils::format_datetime;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Notes {
    pub notes: Vec<Note>,

    #[serde(skip)]
    task_text_entry: String,
    #[serde(skip)]
    deleting_note_idx: Option<usize>,
    #[serde(skip)]
    opened_note_idxs: Vec<usize>,
}

impl Notes {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn delete_note_confirmation(&mut self, ui: &mut Ui, task_idx: usize) {
        ui.with_layout(Layout::top_down(Center), |ui| {
            ui.heading("Are you sure?");
            ui.label(format!("About to delete \"{}\"", self.notes[task_idx].title));
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
                    self.notes.remove(task_idx);
                    self.deleting_note_idx = None;
                }
                if ui.button(RichText::new("Cancel")).clicked() {
                    self.deleting_note_idx = None;
                }
            });
        });
    }

    fn show_note_window(&mut self, ui: &mut Ui, task_idx: usize) {
        if ui.text_edit_multiline(&mut self.notes[task_idx].content).changed() {
            self.notes[task_idx].edited_at = Some(Local::now());
        };
    }

    fn show_note(&mut self, ui: &mut Ui, note_idx: usize) {
        let note = &mut self.notes[note_idx];
        ui.horizontal(|ui| {
            if ui.small_button("Open").clicked() && self.opened_note_idxs.iter().position(|x| *x == note_idx).is_none() {
                self.opened_note_idxs.push(note_idx);
            };
            ui.heading(&note.title);
            ui.with_layout(Layout::right_to_left(Center), |ui| {
                if ui.small_button("🗑").clicked() && self.deleting_note_idx.is_none() {
                    self.deleting_note_idx = Some(note_idx);
                };
                ui.with_layout(Layout::top_down(Max), |ui| {
                    ui.small(format!("created {}", format_datetime(note.created_at)));
                    if let Some(timestamp) = note.edited_at {
                        ui.small(format!("edited {}", format_datetime(timestamp)));
                    }
                });
                let text_edit = TextEdit::singleline(&mut note.content)
                    .hint_text("*empty*")
                    .interactive(false);
                ui.add_sized(ui.available_size(), text_edit);
            });
        });
    }
}

impl eframe::App for Notes {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Notes");
            ui.horizontal_top(|ui| {
                let text_edit = TextEdit::singleline(&mut self.task_text_entry).hint_text("Title");
                let resp = ui.add(text_edit);
                if (resp.lost_focus()
                    && ui.input().key_pressed(egui::Key::Enter))
                   || ui.button("Create a note").clicked()
                {
                    self.notes.push(Note::new(self.task_text_entry.clone()));
                    self.task_text_entry.clear();
                }
            });
            for task_idx in 0..self.notes.len() {
                ui.separator();
                self.show_note(ui, task_idx);
            }
            ui.separator();
            egui::warn_if_debug_build(ui);
        });
        if let Some(task_idx) = self.deleting_note_idx {
            egui::Window::new("Delete note")
                .default_width(100.)
                .default_height(200.)
                .show(ctx, |ui| {
                    self.delete_note_confirmation(ui, task_idx);
                });
        }
        for note_idx in self.opened_note_idxs.clone() {
            let mut open = true;
            egui::Window::new(&self.notes[note_idx].title.clone())
                .auto_sized()
                .scroll2([true, true])
                .open(&mut open)
                .show(ctx, |ui| {
                    self.show_note_window(ui, note_idx);
                });
            if !open {
                let idx = self.opened_note_idxs.iter().position(|x| *x == note_idx).unwrap();
                self.opened_note_idxs.remove(idx);
            }
        }
    }
}

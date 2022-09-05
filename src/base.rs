#[cfg(target_arch = "wasm32")]
use std::time::Duration;

use egui::{ScrollArea, SidePanel};

use crate::Tasks;

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq)]
enum SelectedApp {
    #[default]
    Tasks
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct VigilantDoodle {
    selected_app: SelectedApp,
    tasks: Tasks,

    #[serde(skip)]
    is_setup: bool,
}

impl VigilantDoodle {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VigilantDoodle {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.is_setup {
            ctx.set_pixels_per_point(2.);
            self.is_setup = true;
        }

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Apps");
            ScrollArea::vertical().show(ui, |ui| {
                ui.selectable_value(&mut self.selected_app, SelectedApp::Tasks, "✏ Tasks")
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            match self.selected_app {
                SelectedApp::Tasks => self.tasks.update(ctx, _frame)
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn auto_save_interval(&self) -> Duration {
        // on wasm save on shutdown doesn't work for me
        Duration::from_secs(5)
    }
}

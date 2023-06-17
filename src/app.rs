use eframe::egui::plot::{Line, Plot};
use eframe::egui::{CentralPanel, Context, Key, KeyboardShortcut, Modifiers};
use tracing::info;

use std::sync::{Arc, Mutex};

use crate::measurements::Measurements;

#[derive(Debug)]
pub struct App {
    measurements: Arc<Mutex<Measurements>>,
    include_y: Option<Vec<f64>>,
    savable: bool,
}

impl App {
    pub fn new(window_size: f64, include_y: Option<Vec<f64>>, savable: bool) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Measurements::new(window_size))),
            include_y,
            savable,
        }
    }

    pub fn measurements(&self) -> Arc<Mutex<Measurements>> {
        self.measurements.clone()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut plot = Plot::new("measurements");

            if let Some(ys) = &self.include_y {
                for y in ys.iter() {
                    plot = plot.include_y(*y);
                }
            }

            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.measurements.lock().unwrap().values()));
            });
        });
        ctx.request_repaint();

        if self.savable {
            ctx.input_mut(|i| {
                if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::S)) {
                    info!("Shortcut pressed!");
                }
            });
        }
    }
}

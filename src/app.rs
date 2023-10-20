use chrono::Local;
use eframe::egui::plot::{Line, Plot};
use eframe::egui::{CentralPanel, Context, Key, KeyboardShortcut, Modifiers};
use eframe::epaint::ColorImage;
use tracing::{info, warn};

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

use crate::measurements::Measurements;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct App {
    measurements: Arc<Mutex<Measurements>>,
    include_y: Option<Vec<f64>>,
    savable: bool,
    save_current_plot: bool,
}

impl App {
    pub fn new(window_size: f64, include_y: Option<Vec<f64>>, savable: bool) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Measurements::new(window_size))),
            include_y,
            savable,
            save_current_plot: false,
        }
    }

    pub fn measurements(&self) -> Arc<Mutex<Measurements>> {
        self.measurements.clone()
    }

    pub fn save_current_plot(&self, frame: &ColorImage) -> Result<()> {
        let img_path = format!(
            "{}/imgs/{}.png",
            env::current_dir()?.display(),
            Local::now().format("%g%m%y_%H%M%S")
        );

        let file = File::create(img_path)?;
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, frame.width() as u32, frame.height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(frame.as_raw())?;

        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut plot = Plot::new("measurements");

            if let Some(ys) = &self.include_y {
                for y in ys.iter() {
                    plot = plot.include_y(*y);
                }
            }

            plot.show(ui, |plot_ui| {
                for values in self.measurements.lock().unwrap().values() {
                    plot_ui.line(Line::new(values));
                }
            });
        });

        ctx.request_repaint();

        if self.savable {
            ctx.input_mut(|i| {
                if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::S)) {
                    self.save_current_plot = true;
                    frame.request_screenshot();
                }
            });
        }
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], frame: &eframe::Frame) {
        if self.save_current_plot {
            if let Some(screenshot) = frame.screenshot() {
                match self.save_current_plot(&screenshot) {
                    Ok(_) => info!("Current plot saved successfully"),
                    Err(e) => warn!("Can not save current plot: {}", e),
                };
            }
        }
    }
}

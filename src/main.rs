use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{command, Parser};
use eframe::NativeOptions;
use egui::plot::{Line, Plot};

use self::measurements::Measurements;

mod measurements;

/// A simple tool to monitor and plot values
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum seconds to display before the screen starts scrolling
    #[arg(short, long)]
    window: f64,
}

#[derive(Debug)]
struct App {
    measurements: Arc<Mutex<Measurements>>,
}

impl App {
    fn new(window_size: f64) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Measurements::new(window_size))),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");

            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.measurements.lock().unwrap().values()));
            });
        });
        ctx.request_repaint();
    }
}

fn main() {
    let args = Args::parse();
    let native_options = NativeOptions::default();

    let app = App::new(args.window);
    let ui_measurement = app.measurements.clone();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(value) => ui_measurement.lock().unwrap().append_value_str(&value),
                Err(_) => return,
            }
        }
    });

    eframe::run_native(
        "Serial Monitoring App",
        native_options,
        Box::new(|_| Box::new(app)),
    )
    .unwrap();
}

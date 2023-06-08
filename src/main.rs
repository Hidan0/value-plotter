use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{command, Parser};
use eframe::egui::plot::{Line, Plot};
use eframe::NativeOptions;

use self::measurements::Measurements;

mod measurements;

/// A simple tool to monitor and plot values
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum x to display before the screen starts scrolling
    #[arg(short, long, default_value_t = 2000.)]
    window: f64,

    /// y values to include in the plot
    #[arg(short, long)]
    include_y: Option<Vec<f64>>,
}

#[derive(Debug)]
struct App {
    measurements: Arc<Mutex<Measurements>>,
    include_y: Option<Vec<f64>>,
}

impl App {
    fn new(window_size: f64, include_y: Option<Vec<f64>>) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Measurements::new(window_size))),
            include_y,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
    }
}

fn main() {
    let args = Args::parse();
    let native_options = NativeOptions::default();

    let app = App::new(args.window, args.include_y);
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

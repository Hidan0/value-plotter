use std::io::BufRead;
use std::thread;

use clap::{command, Parser};
use eframe::NativeOptions;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use crate::app::App;

mod app;
mod measurements;

/// A simple tool to monitor and plot values
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum x to display before the screen starts scrolling
    #[arg(short, long, default_value_t = 1000)]
    window: usize,

    /// y values to include in the plot
    #[arg(short, long)]
    include_y: Option<Vec<f64>>,

    /// Enables the ability to save a screenshot of the current plot
    #[arg(short, long, default_value_t = false)]
    savable: bool,
}

fn main() {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subcriber failed");

    let app = App::new(args.window, args.include_y, args.savable);
    let ui_measurement = app.measurements();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(value) => {
                    if let Some(values) = parse_input(value) {
                        ui_measurement.lock().unwrap().append_value(values);
                    }
                }
                Err(_) => {
                    error!("Failed to read line");
                    break;
                }
            }
        }
    });

    info!("Main thread started");
    match eframe::run_native(
        "Serial Monitoring App",
        NativeOptions::default(),
        Box::new(|_| Box::new(app)),
    ) {
        Ok(_) => {}
        Err(e) => error!("Main thread crashed: {}", e),
    }
}

fn parse_input(input: String) -> Option<Vec<f64>> {
    if input.is_empty() {
        return None;
    }

    let mut out = Vec::new();

    for val in input.split(',') {
        match val.parse::<f64>() {
            Ok(val) => out.push(val),
            Err(_) => {
                warn!("Failed to parse value: {}", val);
                return None;
            }
        }
    }

    Some(out)
}

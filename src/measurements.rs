use std::collections::VecDeque;
use std::time::Instant;

use eframe::egui::plot::PlotPoints;

/// One second is equal to one thousand milliseconds
const TO_SECONDS: f64 = 1000.;

#[derive(Debug)]
pub struct Measurements {
    values: VecDeque<[f64; 2]>,
    start: Instant,
    last_x_value: f64,
    pub window_size: f64,
}

impl Measurements {
    pub fn new(window_size: f64) -> Self {
        Self {
            values: VecDeque::default(),
            start: Instant::now(),
            last_x_value: 0.,
            window_size,
        }
    }

    pub fn append_value(&mut self, v: f64) {
        if self.values.is_empty() {
            self.start = Instant::now();
        }

        let x = self.start.elapsed().as_millis() as f64 / TO_SECONDS;

        self.last_x_value += x;
        let min_x = self.last_x_value - self.window_size;

        self.values.push_back([self.last_x_value, v]);

        while let Some(value) = self.values.front() {
            if value[0] < min_x {
                self.values.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn values(&self) -> PlotPoints {
        PlotPoints::from_iter(self.values.iter().copied())
    }
}

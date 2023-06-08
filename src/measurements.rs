use std::collections::VecDeque;
use std::time::Instant;

use egui::plot::PlotPoints;

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
            window_size: window_size / TO_SECONDS,
        }
    }

    pub fn append_value(&mut self, v: f64) {
        let x = self.start.elapsed().as_millis() as f64 / TO_SECONDS;

        let min_x = x - self.window_size;
        self.values.push_back([self.last_x_value + x, v]);
        self.last_x_value += x;

        while let Some(value) = self.values.front() {
            if value[0] < min_x {
                self.values.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn append_value_str(&mut self, s: &str) {
        let v = s.parse::<f64>().unwrap();
        self.append_value(v)
    }

    pub fn values(&self) -> PlotPoints {
        PlotPoints::from_iter(self.values.iter().copied())
    }
}

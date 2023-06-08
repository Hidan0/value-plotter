use std::collections::VecDeque;
use std::time::Instant;

use egui::plot::PlotPoints;

#[derive(Debug)]
pub struct Measurements {
    values: VecDeque<[f64; 2]>,
    start: Instant,
    last_x_value: f64,
}

impl Measurements {
    pub fn new() -> Self {
        Self {
            values: VecDeque::default(),
            start: Instant::now(),
            last_x_value: 0.,
        }
    }

    pub fn append_value(&mut self, v: f64) {
        let elapsed = self.start.elapsed().as_millis() as f64 / 1000.;
        self.values.push_back([self.last_x_value + elapsed, v]);
        self.last_x_value += elapsed;
    }

    pub fn append_value_str(&mut self, s: &str) {
        let v = s.parse::<f64>().unwrap();
        self.append_value(v)
    }

    pub fn values(&self) -> PlotPoints {
        PlotPoints::from_iter(self.values.iter().copied())
    }
}

use std::collections::VecDeque;
use std::time::Instant;

use eframe::egui::plot::PlotPoints;

/// One second is equal to one thousand milliseconds
const TO_SECONDS: f64 = 1000.;

#[derive(Debug)]
pub struct Measurements {
    values: Vec<VecDeque<[f64; 2]>>,
    start: Instant,
    last_x_value: f64,
    pub window_size: f64,
}

impl Measurements {
    pub fn new(window_size: f64) -> Self {
        Self {
            values: Vec::new(),
            start: Instant::now(),
            last_x_value: 0.,
            window_size,
        }
    }

    pub fn append_value(&mut self, vs: Vec<f64>) {
        if self.values.is_empty() {
            self.start = Instant::now();
            if self.values.len() < vs.len() {
                for _ in 0..vs.len() {
                    self.values.push(VecDeque::new());
                }
            }
        }

        let x = self.start.elapsed().as_millis() as f64 / TO_SECONDS;

        self.last_x_value += x;
        let min_x = self.last_x_value - self.window_size;

        vs.iter().enumerate().for_each(|(i, v)| {
            self.values
                .get_mut(i)
                .unwrap()
                .push_back([self.last_x_value, *v])
        });

        for values in self.values.iter_mut() {
            while let Some(value) = values.front() {
                if value[0] < min_x {
                    values.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    pub fn values(&self) -> Vec<PlotPoints> {
        let mut out = Vec::new();
        for values in self.values.iter() {
            out.push(PlotPoints::from_iter(values.iter().copied()));
        }
        out
    }
}

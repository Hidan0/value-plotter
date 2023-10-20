use std::collections::VecDeque;
use std::time::Instant;

use eframe::egui::plot::PlotPoints;

#[derive(Debug)]
pub struct Measurements {
    values: Vec<VecDeque<[f64; 2]>>,
    start: Instant,
    last_x_value: f64,
    pub window_size: usize,
}

impl Measurements {
    pub fn new(window_size: usize) -> Self {
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

        let x = self.start.elapsed().as_millis() as f64 / 1000.;
        self.last_x_value += x;

        vs.iter().enumerate().for_each(|(i, v)| {
            self.values
                .get_mut(i)
                .unwrap()
                .push_back([self.last_x_value, *v])
        });

        for values in self.values.iter_mut() {
            if values.len() > self.window_size {
                values.pop_front();
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

use std::collections::VecDeque;

#[derive(Debug)]
pub struct RollingStats {
    window_size: usize,
    endianness: Endianness,
    values: VecDeque<i32>,
    sum: i64,
    sum_of_squares: i64,
}

#[derive(Debug)]
pub enum Endianness {
    Little,
    Big,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Big
    }
}

impl RollingStats {
    pub fn new(window_size: usize, endianness: Endianness) -> Self {
        assert!(window_size > 0, "Window size must be greater than 0");

        RollingStats {
            window_size,
            endianness,
            values: VecDeque::with_capacity(window_size),
            sum: 0,
            sum_of_squares: 0,
        }
    }
    pub fn add_sample(&mut self, value: i32) {
        if self.values.len() == self.window_size {
            if let Some(removed) = self.values.pop_front() {
                self.sum -= removed as i64;
                self.sum_of_squares -= (removed as i64).pow(2);
            }
        }
        self.values.push_back(value);
        self.sum += value as i64;
        self.sum_of_squares += (value as i64).pow(2);
    }
    pub fn get_mean(&self) -> f32 {
        let count = self.values.len();
        if count == 0 {
            return 0.0;
        }
        self.sum as f32 / count as f32
    }
    pub fn get_std_dev(&self) -> f32 {
        let count = self.values.len();
        if count < 2 {
            return 0.0;
        }
        let mean = self.get_mean();
        let variance = (self.sum_of_squares as f32 / count as f32) - (mean * mean);
        variance.sqrt()
    }
}

impl Default for RollingStats {
    fn default() -> Self {
        RollingStats::new(3, Endianness::default())
    }
}

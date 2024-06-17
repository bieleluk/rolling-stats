use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rand_distr::{Distribution, Normal};
use std::collections::VecDeque;
use std::io::{self, Write};

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
    fn add_sample(&mut self, value: i32) {
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
    pub fn mean(&self) -> f32 {
        let count = self.values.len();
        if count == 0 {
            return 0.0;
        }
        self.sum as f32 / count as f32
    }
    pub fn sample(&self) -> f32 {
        let mean = self.mean();
        let std_dev = self.std_dev();
        if std_dev == 0.0 {
            return mean;
        }
        // Can not fail, because std_Dev is always non-negative
        let normal = Normal::new(mean, std_dev).unwrap();
        normal.sample(&mut rand::thread_rng())
    }
    pub fn std_dev(&self) -> f32 {
        let count = self.values.len();
        if count < 2 {
            return 0.0;
        }
        let mean = self.mean();
        let variance = (self.sum_of_squares as f32 / count as f32) - (mean * mean);
        variance.sqrt()
    }
    fn read_i32_from_bytes(&self, bytes: &[u8]) -> i32 {
        match self.endianness {
            Endianness::Little => LittleEndian::read_i32(bytes),
            Endianness::Big => BigEndian::read_i32(bytes),
        }
    }
}

impl Default for RollingStats {
    fn default() -> Self {
        RollingStats::new(3, Endianness::default())
    }
}

impl Write for RollingStats {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_consumed = 0;
        for chunk in buf.chunks(4) {
            if chunk.len() == 4 {
                let value = self.read_i32_from_bytes(chunk);
                self.add_sample(value);
                bytes_consumed += 4;
            } else {
                println!(
                    "Incomplete byte chunk of size {}, skipping for now",
                    chunk.len()
                );
            }
        }
        Ok(bytes_consumed)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

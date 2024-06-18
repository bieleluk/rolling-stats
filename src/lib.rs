#![no_std]

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use heapless::{Deque, Vec};
use rand_distr::{Distribution, Normal};

#[derive(Debug)]
pub struct RollingStats {
    // Size of the rolling window
    window_size: usize,
    // Endianness for interpreting bytes
    endianness: Endianness,
    // Queue of values in the rolling window
    values: Deque<i32, 10>,
    // Buffer for leftover bytes
    remainder: Vec<u8, 4>,
    // Sum of values in the window
    sum: i64,
    // Sum of squares of values in the window
    sum_of_squares: i64,
}

#[derive(Debug, Default, PartialEq)]
pub enum Endianness {
    Little,
    #[default]
    Big,
}

impl RollingStats {
    pub fn new(window_size: usize, endianness: Endianness) -> Self {
        assert!(
            window_size > 0 && window_size <= 10,
            "Window size must be between 1 and 10"
        );

        RollingStats {
            window_size,
            endianness,
            values: Deque::<_, 10>::new(),
            remainder: Vec::<_, 4>::new(),
            sum: 0,
            sum_of_squares: 0,
        }
    }
    fn add_sample(&mut self, value: i32) {
        // Remove the oldest sample if the window is full
        if self.values.len() == self.window_size {
            if let Some(removed) = self.values.pop_front() {
                self.sum -= removed as i64;
                self.sum_of_squares -= (removed as i64).pow(2);
            }
        }
        // Add new sample to the back and update sum/squresum
        let _ = self.values.push_back(value);
        self.sum += value as i64;
        self.sum_of_squares += (value as i64).pow(2);
    }
    pub fn mean(&self) -> f32 {
        let count = self.values.len();
        // In case of no data, mean is 0
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
        // Can not fail, because std_dev is always non-negative
        let normal = Normal::new(mean, std_dev).unwrap();
        normal.sample(&mut rand::thread_rng())
    }
    pub fn std_dev(&self) -> f32 {
        let count = self.values.len();
        // In case of 1 or 0 data samples, std dev is 0
        if count < 2 {
            return 0.0;
        }
        let mean = self.mean();
        let variance = (self.sum_of_squares as f32 / count as f32) - (mean * mean);
        variance.sqrt()
    }
    fn read_i32_from_bytes(&self, bytes: &[u8]) -> i32 {
        match self.endianness {
            Endianness::Little => LittleEndian::read_i32(&bytes[0..4]),
            Endianness::Big => BigEndian::read_i32(&bytes[0..4]),
        }
    }
    pub fn write(&mut self, buf: &[u8]) -> usize {
        let mut bytes_consumed = 0;
        let mut start = 0;
        if !self.remainder.is_empty() {
            start = 4 - self.remainder.len();
            if buf.len() >= start {
                let _ = self.remainder.extend_from_slice(&buf[0..start]);
                let value = self.read_i32_from_bytes(&self.remainder);
                self.add_sample(value);
                self.remainder.clear();
                bytes_consumed += start;
            } else {
                let _ = self.remainder.extend_from_slice(buf);
                return buf.len();
            }
        }
        for chunk in buf[start..].chunks(4) {
            if chunk.len() == 4 {
                let value = self.read_i32_from_bytes(chunk);
                self.add_sample(value);
                bytes_consumed += 4;
            } else {
                let _ = self.remainder.extend_from_slice(chunk);
                bytes_consumed += chunk.len();
            }
        }
        bytes_consumed
    }
}

impl Default for RollingStats {
    fn default() -> Self {
        RollingStats::new(3, Endianness::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let mut stats = RollingStats::default();
        assert_eq!(stats.endianness, Endianness::Big);
        stats.add_sample(1);
        stats.add_sample(2);
        stats.add_sample(3);
        assert_eq!(stats.values.len(), 3);
        stats.add_sample(4);
        assert_eq!(stats.values.len(), 3);
    }

    #[test]
    fn test_win_size() {
        let mut stats = RollingStats::new(4, Endianness::Little);
        stats.add_sample(1);
        stats.add_sample(2);
        stats.add_sample(3);
        stats.add_sample(4);
        assert_eq!(stats.values.len(), 4);
        stats.add_sample(5);
        assert_eq!(stats.values.len(), 4);
    }

    #[test]
    fn test_sums_and_squaresums() {
        let mut stats = RollingStats::new(3, Endianness::Big);
        stats.add_sample(1);
        stats.add_sample(2);
        stats.add_sample(3);
        assert_eq!(stats.sum, 6);
        assert_eq!(stats.sum_of_squares, 14);
    }

    #[test]
    fn test_mean() {
        let mut stats = RollingStats::new(3, Endianness::Big);
        stats.add_sample(1);
        stats.add_sample(2);
        stats.add_sample(3);
        assert_eq!(stats.mean(), 2.0);
        stats.add_sample(4);
        assert_eq!(stats.mean(), 3.0);
    }

    #[test]
    fn test_std_dev() {
        let mut stats = RollingStats::new(3, Endianness::Big);
        stats.add_sample(1);
        stats.add_sample(2);
        stats.add_sample(3);
        let std_dev = stats.std_dev();

        assert!((std_dev - 0.81649658092773).abs() < 0.000001);

        stats.add_sample(4);
        let std_dev = stats.std_dev();

        assert!((std_dev - 0.81649658092773).abs() < 0.000001);
    }

    #[test]
    fn test_sample_one_sample() {
        let mut stats = RollingStats::new(3, Endianness::Big);
        stats.add_sample(1);
        let value = stats.sample();

        assert!((value - 1.0).abs() < 0.000001);
    }

    #[test]
    fn test_sample_zero_samples() {
        let stats = RollingStats::new(3, Endianness::Big);
        let value = stats.sample();

        assert!((value - 0.0).abs() < 0.000001);
    }

    #[test]
    fn test_read_i32_le() {
        let stats = RollingStats::new(3, Endianness::Little);
        let bytes = [1, 0, 0, 0];

        assert_eq!(stats.read_i32_from_bytes(&bytes), 1);
    }

    #[test]
    fn test_read_i32_be() {
        let stats = RollingStats::new(3, Endianness::Big);
        let bytes = [0, 0, 0, 1];

        assert_eq!(stats.read_i32_from_bytes(&bytes), 1);
    }

    #[test]
    fn test_write_no_rem() {
        let mut stats = RollingStats::new(3, Endianness::Big);
        let bytes = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        let ret = stats.write(&bytes);

        assert_eq!(ret, 12);
        assert_eq!(stats.values.len(), 3);
        assert_eq!(stats.sum, 6);
        assert_eq!(stats.remainder.len(), 0);
    }

    #[test]
    fn test_write_with_remainder() {
        let mut stats = RollingStats::new(4, Endianness::Big);

        // Addition of several elements with a remainder
        let bytes = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0];
        let ret = stats.write(&bytes);
        assert_eq!(ret, 13);
        assert_eq!(stats.values.len(), 3);
        assert_eq!(stats.sum, 6);
        assert_eq!(stats.remainder.len(), 1);

        // Only extending a reminder
        let bytes = [0];
        let ret = stats.write(&bytes);
        assert_eq!(ret, 1);
        assert_eq!(stats.values.len(), 3);
        assert_eq!(stats.sum, 6);
        assert_eq!(stats.remainder.len(), 2);

        // Cancellation of a reminder
        let bytes = [0, 4];
        let ret = stats.write(&bytes);
        assert_eq!(ret, 2);
        assert_eq!(stats.values.len(), 4);
        assert_eq!(stats.sum, 10);
        assert_eq!(stats.remainder.len(), 0);
    }
}

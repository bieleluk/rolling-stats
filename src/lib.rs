use std::collections::VecDeque;

#[derive(Debug)]
pub struct RollingStats {
    window_size: usize,
    endianness: Endianness,
    values: VecDeque<i32>,
}

#[derive(Debug)]
pub enum Endianness {
    Little,
    Big,
}


impl RollingStats {
    pub fn new(window_size: usize, endianness: Endianness) -> Self {
        assert!(window_size > 0, "Window size must be greater than 0");

        RollingStats {
            window_size,
            endianness,
            values: VecDeque::with_capacity(window_size),
        }
    }
}
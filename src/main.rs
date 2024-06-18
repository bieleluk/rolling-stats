#![no_std]

use rolling_stats::{RollingStats,Endianness};
use libc_print::std_name::println;

fn main() {
    println!("Creating empty default stats");
    let mut stats: RollingStats = RollingStats::default();
    println!("Random sample (0 expected) {}", stats.sample());
    assert_eq!(stats.write(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]), 16);
    assert_eq!(stats.mean(), 3.0);
    println!("{:?}", stats);
    println!("Random sample {}", stats.sample());

    println!("Creating little endian stats with size 4");
    let mut stats: RollingStats = RollingStats::new(4, Endianness::Little);
    assert_eq!(stats.write(&[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]), 16);
    assert_eq!(stats.mean(), 2.5);
    println!("{:?}", stats);

    println!("Creating empty default stats");
    let mut stats: RollingStats = RollingStats::default();
    assert_eq!(stats.write(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0]), 18);
    println!("{:?}", stats);
    assert_eq!(stats.write(&[0]), 1);
    println!("{:?}", stats);
    assert_eq!(stats.write(&[1, 0, 0, 0]), 4);
    println!("{:?}", stats);

}

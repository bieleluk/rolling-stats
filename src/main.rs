use rolling_stats::{RollingStats,Endianness};
use std::io::Write;

fn main() {
    println!("Creating empty default stats");
    let mut stats: RollingStats = RollingStats::default();
    assert!(stats.write(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]).is_ok());
    assert_eq!(stats.mean(), 3.0);
    println!("{:?}", stats);

    println!("Creating little endian stats with size 4");
    let mut stats: RollingStats = RollingStats::new(4, Endianness::Little);
    assert!(stats.write(&[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]).is_ok());
    assert_eq!(stats.mean(), 2.5);
    println!("{:?}", stats);

    println!("Creating empty default stats");
    let mut stats: RollingStats = RollingStats::default();
    assert!(stats.write(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 4]).is_ok());
    assert_eq!(stats.mean(), 2.0);
    println!("{:?}", stats);


}

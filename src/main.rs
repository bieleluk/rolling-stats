use rolling_stats::{RollingStats,Endianness};

fn main() {
    println!("Hello, world!");

    let stats: RollingStats = RollingStats::new(3, Endianness::Big);
    println!("{:?}", stats);
}

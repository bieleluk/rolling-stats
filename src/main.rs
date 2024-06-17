use rolling_stats::{RollingStats,Endianness};

fn main() {
    println!("Creating empty stats with size 2");
    let mut stats: RollingStats = RollingStats::new(2, Endianness::Big);
    println!("Stats for empty struct: {} {}", stats.get_mean(), stats.get_std_dev());
    println!("{:?}", stats);
    println!("Adding 10, expecting 1 element");
    stats.add_sample(10);
    println!("{:?}", stats);
    println!("Stats: {} {}", stats.get_mean(), stats.get_std_dev());
    println!("Adding -20, expecting 2 elements");
    stats.add_sample(-20);
    println!("{:?}", stats);
    println!("Stats: {} {}", stats.get_mean(), stats.get_std_dev());
    println!("Adding 5, expecting 2 elements: -20 and 5");
    stats.add_sample(5);
    println!("{:?}", stats);
    println!("Stats: {} {}", stats.get_mean(), stats.get_std_dev());

}

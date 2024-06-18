# Rolling Stats

`rolling-stats` is a Rust library designed to calculate rolling statistics (mean, standard deviation) over a fixed-size window of integer samples. It also supports reading samples from byte buffers with configurable endianness.

## Features

- **No Standard Library:** Designed to work in `#![no_std]` environments.
- **Fixed-size Window:** Maintains a rolling window of samples with a user-defined size.
- **Endianness Support:** Configurable endianness (big or little) for interpreting byte streams.
- **Statistical Functions:** Computes mean, standard deviation, and can generate samples from a normal distribution based on the rolling statistics.

## Usage

### Building the Library

```bash
cargo build --lib
```

### Generate Documentation

```bash
cargo doc --open
```

### Run tests

```bash
cargo tests
```

### Running sample project

```bash
cargo run
```

### Adding to Your Project

Add `rolling-stats` to your `Cargo.toml` dependencies:

```toml
[dependencies]
rolling-stats = "0.1.0"
```

## Examples

Here are some examples of how to use the rolling-stats library.

### Creating a RollingStats Instance

```rust
use rolling_stats::{RollingStats, Endianness};

let stats = RollingStats::new(3, Endianness::Big);
```

### Writing Samples from Byte Arrays

You can write samples to the RollingStats instance using the write method, which accepts a byte array. The method returns the number of bytes consumed.

```rust
use rolling_stats::{RollingStats, Endianness};

let mut stats = RollingStats::new(3, Endianness::Big);
let bytes = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
let consumed = stats.write(&bytes);
assert_eq!(consumed, 12);
assert_eq!(stats.mean(), 2.0);
```

### Handling Incomplete Numbers

The write method can handle cases where the byte array does not contain a complete number. If the number of bytes in the array is not a multiple of 4 (the size of an i32), the remaining bytes are stored and used in the next call to write.

```rust
use rolling_stats::{RollingStats, Endianness};

let mut stats = RollingStats::new(3, Endianness::Big);
let bytes1 = [0, 0, 0, 1, 0, 0];
let consumed1 = stats.write(&bytes1);
assert_eq!(consumed1, 6);

let bytes2 = [0, 2, 0, 0, 0, 3];
let consumed2 = stats.write(&bytes2);
assert_eq!(consumed2, 6);

assert_eq!(stats.mean(), 2.0);
```

In this example, the first call to write consumes 6 bytes, storing the incomplete 2 bytes of a number in the internal buffer. The second call to write completes this number and continues to process the remaining bytes.

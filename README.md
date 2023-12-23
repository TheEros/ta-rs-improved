# Technical Analysis for Rust (ta)

Technical analysis library for Rust.

- [Getting started](#getting-started)
- [Basic ideas](#basic-ideas)
- [List of indicators](#list-of-indicators)
- [Contributors](#contributors)

## Getting started

Add to you `Cargo.toml`:

```
[dependencies]
ta = { git = "https://github.com/austin-starks/ta-rs-improved" }
```

Example:

```rust
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

let mut ema = ExponentialMovingAverage::new(Duration::seconds(3)).unwrap(); // window size of 3 seconds
let now = Utc::now();

assert_eq!(ema.next((now, 2.0)), 2.0);
assert_eq!(ema.next((now + Duration::seconds(1), 5.0)), 3.5);
assert_eq!(ema.next((now + Duration::seconds(2), 1.0)), 2.25);
assert_eq!(ema.next((now + Duration::seconds(3), 6.25)), 4.25);
```

See more in the examples [here](https://github.com/greyblake/ta-rs/tree/master/examples).
Check also the [documentation](https://docs.rs/ta).

## Basic ideas

Indicators typically implement the following traits:

- `Next<T>` (often `Next<f64>`) - to feed and get the next value
- `Reset` - to reset an indicator
- `Debug`
- `Display`
- `Default`
- `Clone`

## List of indicators

So far there are the following indicators available.

- Trend
  - Exponential Moving Average (EMA)
  - Simple Moving Average (SMA)
- Oscillators
  - Relative Strength Index (RSI)
- Other
  - Minimum
  - Maximum
  - Standard Deviation (SD)
  - Mean Absolute Deviation (MAD)
  - Bollinger Bands (BB)
  - Rate of Change (ROC)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - original creator of ta-rs.
- [austin-starks](https://github.com/austin-starks) Austin Starks – the creator of this repo

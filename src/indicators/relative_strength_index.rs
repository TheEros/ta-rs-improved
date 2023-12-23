use std::collections::VecDeque;
use std::fmt;

use crate::errors::Result;
use crate::indicators::ExponentialMovingAverage as Ema;
use crate::{Next, Reset};
use chrono::{DateTime, Duration, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[doc(alias = "RSI")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    duration: Duration,
    up_ema_indicator: Ema,
    down_ema_indicator: Ema,
    window: VecDeque<(DateTime<Utc>, f64)>, // Store tuples of (timestamp, value)
    prev_val: Option<f64>,
}

impl RelativeStrengthIndex {
    pub fn new(duration: Duration) -> Result<Self> {
        Ok(Self {
            duration,
            up_ema_indicator: Ema::new(duration)?,
            down_ema_indicator: Ema::new(duration)?,
            window: VecDeque::new(),
            prev_val: None,
        })
    }

    fn remove_old_data(&mut self, current_time: DateTime<Utc>) {
        while self
            .window
            .front()
            .map_or(false, |(time, _)| *time <= current_time - self.duration)
        {
            self.window.pop_front();
        }
    }
}

impl Next<f64> for RelativeStrengthIndex {
    type Output = f64;

    fn next(&mut self, (timestamp, value): (DateTime<Utc>, f64)) -> Self::Output {
        self.remove_old_data(timestamp);
        self.window.push_back((timestamp, value));

        let mut up = 0.0;
        let mut down = 0.0;

        if let Some(prev_val) = self.prev_val {
            if value > prev_val {
                up = value - prev_val;
            } else {
                down = prev_val - value;
            }
        }

        self.prev_val = Some(value);
        let up_ema = self.up_ema_indicator.next((timestamp, up));
        let down_ema = self.down_ema_indicator.next((timestamp, down));

        if up_ema + down_ema == 0.0 {
            50.0 // To avoid division by zero, return a neutral value
        } else {
            100.0 * up_ema / (up_ema + down_ema)
        }
    }
}

impl Reset for RelativeStrengthIndex {
    fn reset(&mut self) {
        self.window.clear();
        self.prev_val = None;
        self.up_ema_indicator.reset();
        self.down_ema_indicator.reset();
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new(Duration::days(14)).unwrap()
    }
}

impl fmt::Display for RelativeStrengthIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSI({:?} days)", self.duration.num_days())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;
    use chrono::{TimeZone, Utc};

    test_indicator!(RelativeStrengthIndex);

    #[test]
    fn test_new() {
        assert!(RelativeStrengthIndex::new(Duration::days(0)).is_err());
        assert!(RelativeStrengthIndex::new(Duration::days(1)).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rsi = RelativeStrengthIndex::new(Duration::days(3)).unwrap();
        let timestamp = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        assert_eq!(rsi.next((timestamp, 10.0)), 50.0);
        assert_eq!(
            rsi.next((timestamp + Duration::days(1), 10.5)).round(),
            86.0
        );
        assert_eq!(
            rsi.next((timestamp + Duration::days(2), 10.0)).round(),
            35.0
        );
        assert_eq!(rsi.next((timestamp + Duration::days(3), 9.5)).round(), 16.0);
    }

    #[test]
    fn test_reset() {
        let mut rsi = RelativeStrengthIndex::new(Duration::days(3)).unwrap();
        let timestamp = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        assert_eq!(rsi.next((timestamp, 10.0)), 50.0);
        assert_eq!(
            rsi.next((timestamp + Duration::days(1), 10.5)).round(),
            86.0
        );

        rsi.reset();
        assert_eq!(rsi.next((timestamp, 10.0)).round(), 50.0);
        assert_eq!(
            rsi.next((timestamp + Duration::days(1), 10.5)).round(),
            86.0
        );
    }

    #[test]
    fn test_default() {
        RelativeStrengthIndex::default();
    }

    #[test]
    fn test_display() {
        let rsi = RelativeStrengthIndex::new(Duration::days(16)).unwrap();
        assert_eq!(format!("{}", rsi), "RSI(16 days)");
    }
}

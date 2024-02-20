use rand::prelude::*;

use crate::interval::Interval;

pub fn random_f64(interval: Interval) -> f64 {
    interval.min + (interval.max - interval.min) * random::<f64>()
}
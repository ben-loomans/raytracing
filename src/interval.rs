pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn contains(&self, item: f64) -> bool {
        self.min <= item && item <= self.max
    }
    
    pub fn surrounds(&self, item: f64) -> bool {
        self.min < item && item < self.max
    }

    pub fn clamp(&self, item: f64) -> f64 {
        if item < self.min {return self.min}
        if item > self.max {return self.max}
        return item
    }
}
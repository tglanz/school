use *;

#[derive(Debug)]
pub enum Aggregation {
    Maximum,
    Minimum,
    Average
}

#[derive(Debug)]
pub struct PoolingLayer {
    aggregation: Aggregation,
    size: Dimensions2d,
    strides: Strides2d,
}

impl PoolingLayer {
    pub fn new(aggregation: Aggregation, size: Dimensions2d, strides: Strides2d) -> Self {
        Self { aggregation, size, strides }
    }
}
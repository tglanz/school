use *;
use layers::pooling::{PoolingLayer};

#[derive(Debug)]
pub struct Convolution2dLayer {
    input_dimensions: Dimensions3d,

    filters_dimensions: Dimensions3d,
    filters_count: usize,

    output_dimensions: Dimensions3d,

    activation: Option<ActivationType>,
    pooling: Option<PoolingLayer>,
}

impl Convolution2dLayer {
    pub fn new(input_dimensions: Dimensions3d, filters_dimensions: Dimensions3d, filters_count: usize) -> Self {
        Self {
            input_dimensions,
            filters_dimensions,
            filters_count,
            output_dimensions: (input_dimensions.0, input_dimensions.1, filters_count),
            pooling: None,
            activation: None,
        }
    }

    pub fn with_activation(mut self, activation: ActivationType) -> Self {
        self.activation = Some(activation);
        self
    }

    pub fn with_pooling(mut self, pooling: PoolingLayer) -> Self {
        self.pooling = Some(pooling);
        self
    }
}
use *;

#[derive(Debug)]
pub struct FullyConnectedLayer {
    input_size: Dimensions1d,
    output_size: Dimensions1d,
    bias_size: Dimensions1d,
    weights_dimensions: Dimensions2d,

    activation: Option<ActivationType>
}

impl FullyConnectedLayer {
    pub fn new(input_size: Dimensions1d, output_size: Dimensions1d) -> Self {
        Self {
            input_size,
            output_size,
            bias_size: output_size,
            weights_dimensions: (input_size, output_size),

            activation: None
        }
    }

    pub fn with_activation(mut self, activation: ActivationType) -> Self {
        self.activation = Some(activation);
        self
    }
}
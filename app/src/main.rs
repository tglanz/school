extern crate neural_networks;

use neural_networks::layers::fully_connected::{FullyConnectedLayer};
use neural_networks::layers::convolution2d::{Convolution2dLayer};
use neural_networks::layers::pooling::{PoolingLayer, Aggregation};
use neural_networks::{ActivationType};

fn main() {
    let fc = FullyConnectedLayer::new(3, 6)
        .with_activation(ActivationType::ReLU);

    let conv2d = Convolution2dLayer::new((3, 3, 3), (3, 3, 3), 10)
        .with_pooling(PoolingLayer::new(
            Aggregation::Maximum,
            (2, 2),
            (1, 1))
        )
        .with_activation(ActivationType::Sigmoid);

    println!("Hello, world!");
    println!("We have constructed the following layer definitions");
    println!(" >> {:?}", fc);
    println!(" >> {:?}", conv2d);
}

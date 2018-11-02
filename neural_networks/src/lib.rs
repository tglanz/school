#![allow(dead_code)]

/// neural networks
/// 
/// Contains definitions to operate on neural networks
/// 
/// By design, the layers by themselves are just data containers,
/// the execution of the networks is out of their scope.
/// Meaning that a layer will act as a recipe structure of how it should
/// be constructed, and run during actual execution.
/// The intention is to break apart the network definition and it's execution by
/// the compute kernels of OpenCL and/or Compute shaders of Vulkan
/// 
/// # Todo
/// Think of a way to structure the projects; for example
/// | -- neural_networks
/// |  | -- graph
/// |  | -- | builder, ..., reconciler, etc..
/// |  | -- layers
/// |  | -- | fully_connected, ..., convolution, etc..
/// ... computation probably on a different package

pub mod layers;

#[derive(Debug)]
pub enum ActivationType {
    Identity, // do we need this? can just handle no activation
    Sigmoid,
    ReLU,
    // Leaky
}

pub type Dimensions1d = (usize);
pub type Dimensions2d = (usize, usize);
pub type Dimensions3d = (usize, usize, usize);

pub type Strides1d = (usize);
pub type Strides2d = (usize, usize);
pub type Strides3d = (usize, usize, usize);
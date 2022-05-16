use custos::{Matrix, GenericOCL, number::Float, cpu::TBlas};
use gradients::{Linear, SampleUniform, Softmax, ReLU, Param, GetParam};

pub struct Network {
    pub layers: Vec<Box<dyn Layer<f32>>>,
}

impl Default for Network {
    fn default() -> Self {
        Self { layers: Default::default() }
    }
}

impl Network {
    pub fn new() -> Network {
        Network { layers: Default::default() }
    }

    pub fn add(&mut self, layer: Box<dyn Layer<f32>>) {
        self.layers.push(layer);
    }

    pub fn from_layers(layers: Vec<Box<dyn Layer<f32>>>) -> Network {
        Network { layers }
    }

    pub fn forward(&mut self, mut inputs: Matrix<f32>) -> Matrix<f32> {
        for layer in &mut self.layers {
            inputs = layer.forw(inputs);
        }
        inputs
    }

    pub fn backward(&mut self, mut grad: Matrix<f32>) -> Matrix<f32> {
        for layer in &mut self.layers {
            grad = layer.forw(grad);
        }
        grad
    }
    pub fn params(&mut self) -> Vec<Param<f32>> {
        self.layers
            .iter_mut()
            .flat_map(|layer| layer.params())
            .collect()
    }
}

pub trait Layer<T> {
    fn forw(&mut self, forward: Matrix<T>) -> Matrix<T>;
    fn backw(&mut self, grad: Matrix<T>) -> Matrix<T>;
    fn params(&mut self) -> Option<Param<T>>;
}

impl<T: GenericOCL+Float+TBlas+SampleUniform> Layer<T> for Linear<T> {
    fn forw(&mut self, forward: Matrix<T>) -> Matrix<T> {
        self.forward(forward)
    }

    fn backw(&mut self, grad: Matrix<T>) -> Matrix<T> {
        self.backward(grad)
    }

    fn params(&mut self) -> Option<Param<T>> {
        self.get_params()
    }
}

impl<T: GenericOCL+Float+TBlas+SampleUniform> Layer<T> for Softmax<T> {
    fn forw(&mut self, forward: Matrix<T>) -> Matrix<T> {
        self.forward(forward)
    }

    fn backw(&mut self, grad: Matrix<T>) -> Matrix<T> {
        self.backward(grad)
    }

    fn params(&mut self) -> Option<Param<T>> {
        self.get_params()
    }
}

impl<T: GenericOCL+Float+TBlas+SampleUniform> Layer<T> for ReLU<T> {
    fn forw(&mut self, forward: Matrix<T>) -> Matrix<T> {
        self.forward(forward)
    }

    fn backw(&mut self, grad: Matrix<T>) -> Matrix<T> {
        self.backward(grad)
    }

    fn params(&mut self) -> Option<Param<T>> {
        self.get_params()
    }
}
use crate::variable::Tensor;

#[allow(unused_variables)]
pub trait Layer {
    fn forward(&self, xs: &Tensor) -> Tensor;
    fn backward(&self, xs: &Tensor, ys: &Tensor) -> Tensor {
        panic!("This layer does not support backward propagation");
    }
    fn forward_training(&self, xs: &Tensor) -> Tensor {
        panic!("This layer does not support training");
    }
}

pub struct Relu {}

impl Layer for Relu {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.relu()
    }
}

pub struct LeakyRelu {
    slope: f64,
}

impl Layer for LeakyRelu {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.leaky_relu(0.2, true)
    }
}

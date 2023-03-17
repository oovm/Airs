use airs_types::{Layer, Tensor};

pub struct Relu {}

pub struct LeakyRelu {
    slope: f64,
}

pub struct ParametricRelu {
    alpha: Tensor,
}

pub struct RandomizedRelu {
    lower: f64,
    upper: f64,
}

pub struct Sigmoid {
    alpha: f64,
    beta: f64,
}

impl Layer for Relu {
    fn forward(&self, xs: &Tensor) -> Tensor {
        todo!()
    }
    fn train_forward(&self, xs: &Tensor) -> Tensor {
        todo!()
    }
}

impl Layer for LeakyRelu {
    fn forward(&self, xs: &Tensor) -> Tensor {
        todo!()
    }
}

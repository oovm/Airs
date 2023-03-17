use airs_types::{Layer, Tensor};

pub struct BatchNorm {
    pub gamma: Tensor,
    pub beta: Tensor,
    pub mean: Tensor,
    pub variance: Tensor,
    pub epsilon: f64,
    pub momentum: f64,
    pub is_training: bool,
}

impl Layer for BatchNorm {
    fn forward(&self, xs: &Tensor) -> Tensor {
        todo!()
    }
}

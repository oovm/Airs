use airs_types::Tensor;

pub trait TensorExtension {
    fn relu(&self) -> Self;
    fn leaky_relu(&self, slope: f64) -> Self;
}

impl TensorExtension for Tensor {
    fn relu(&self) -> Self {
        todo!()
    }
    fn leaky_relu(&self, slope: f64) -> Self {
        todo!()
    }
}

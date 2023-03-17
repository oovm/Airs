use crate::Tensor;

#[allow(unused_variables)]
pub trait Layer {
    fn forward(&self, xs: &Tensor) -> Tensor;
    fn train_backward(&self, xs: &Tensor, ys: &Tensor) -> Tensor {
        panic!("This layer does not support backward propagation");
    }
    fn train_forward(&self, xs: &Tensor) -> Tensor {
        self.forward(xs)
    }
}

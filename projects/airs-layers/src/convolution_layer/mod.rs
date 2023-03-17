use airs_types::Tensor;

pub struct ConvolutionND<D> {
    pub weight: Tensor,
    pub bias: Tensor,
    pub stride: Vec<i64>,
    pub padding: Vec<i64>,
    pub dilation: Vec<i64>,
    pub groups: i64,
    pub data_format: D,
}

pub type Convolution1D = ConvolutionND<[i64; 1]>;

pub type Convolution2D = ConvolutionND<[i64; 2]>;

pub type Convolution3D = ConvolutionND<[i64; 2]>;

pub struct ConvolutionConfig {
    pub stride: Vec<i64>,
    pub padding: Vec<i64>,
    pub dilation: Vec<i64>,
    pub groups: i64,
}

impl ConvolutionConfig {
    pub fn build<D>(&self, data_format: D) -> ConvolutionND<D> {
        todo!()
    }
}

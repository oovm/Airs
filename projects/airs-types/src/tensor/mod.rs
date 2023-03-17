use crate::DataType;

pub mod data_type;
pub mod device;
mod from_image;

#[derive(Debug, Default)]
pub struct Tensor {
    kind: DataType,
}
impl Tensor {
    /// Creates a new tensor with the specified data type.
    pub fn get_type(&self) -> DataType {
        self.kind
    }
    /// Sets the data type of the tensor.
    ///
    /// # Arguments
    ///
    /// * `kind`: The data type of the tensor.
    ///
    /// # Examples
    ///
    /// ```
    /// use airs_types::Tensor;
    /// let mut tensor = Tensor::default();
    /// unsafe { tensor.set_type(airs_types::DataType::Float) };
    /// ```
    pub unsafe fn set_type(&mut self, kind: DataType) {
        self.kind = kind;
    }
    pub fn cast_type(&mut self, kind: DataType) {
        self.kind = kind;
    }
    pub fn with_type(mut self, kind: DataType) -> Self {
        self.cast_type(kind);
        self
    }
    pub fn get_bytes(&self) -> &[u8] {
        todo!()
    }
    pub unsafe fn set_bytes(&mut self, bytes: &[u8]) {
        todo!()
    }
    pub fn with_bytes(mut self, bytes: &[u8]) -> Self {
        unsafe { self.set_bytes(bytes) };
        self
    }
}

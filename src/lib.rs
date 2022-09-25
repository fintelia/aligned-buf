const MAX_ALIGN: usize = 16;

pub struct AlignedBuf {
    inner: Box<[u128]>,
    len: usize,
}
impl AlignedBuf {
    pub fn new(len: usize) -> Self {
        Self {
            inner: vec![0u128; len.saturating_add(MAX_ALIGN - 1) / MAX_ALIGN].into(),
            len,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_slice<T: bytemuck::AnyBitPattern>(&self) -> &[T] {
        &bytemuck::cast_slice(&self.inner)[..self.len / std::mem::size_of::<T>()]
    }
    pub fn as_slice_mut<T: bytemuck::NoUninit + bytemuck::AnyBitPattern>(&mut self) -> &mut [T] {
        &mut bytemuck::cast_slice_mut(&mut self.inner)[..self.len / std::mem::size_of::<T>()]
    }
}

use crate::util;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;

/// A resizable wgpu GPU-side buffer, that is only reallocated when required.
pub struct ResizableBuffer<D> {
    /// The handle for the gpu-side buffer.
    buffer: wgpu::Buffer,

    /// The number of used elements in the buffer/array.
    len: usize,

    /// The current maximum capacity of the gpu-side buffer before it needs to be resized.
    capacity: usize,

    /// The wgpu label of the buffer, used for debugging.
    label: String,

    /// The wgpu buffer usages that need to be known when the buffer is reuploaded
    usage: wgpu::BufferUsages,

    /// To ensure that only D types are used with the buffer.
    _array_type: PhantomData<D>,
}

#[allow(dead_code)]
impl<D> ResizableBuffer<D> {
    /// Creates an empty buffer for later use
    /// Note: that wgpu::BufferUsages::COPY_DST is automatically added to usages, as it is required
    /// when reuploading.
    pub fn new(device: &wgpu::Device, buffer_label: &str, usage: wgpu::BufferUsages) -> Self {
        let label = String::from(buffer_label);

        // will always need copy_dst, to allow reuploading without creating a new buffer.
        let usage = usage | wgpu::BufferUsages::COPY_DST;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&label),
            usage,
            size: 0,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            len: 0,
            capacity: 0,
            label,
            usage,
            _array_type: PhantomData,
        }
    }

    /// Creates an initialised buffer with some data
    /// Note: that wgpu::BufferUsages::COPY_DST is automatically added to usages, as it is required
    /// when reuploading.
    pub fn upload(
        device: &wgpu::Device,
        data: &[D],
        buffer_label: &str,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let data_bytes = unsafe { util::slice_as_u8_slice(data) };

        let label = String::from(buffer_label);

        // will always need copy_dst, to allow reuploading without creating a new buffer.
        let usage = usage | wgpu::BufferUsages::COPY_DST;

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&label),
            contents: data_bytes,
            usage,
        });

        Self {
            buffer,
            len: data.len(),
            capacity: data.len(),
            label,
            usage,
            _array_type: PhantomData,
        }
    }

    /// Attempts to upload the provided instances Vec into the previously-allocated gpu
    /// memory. If not enough gpu memory is allocated, then the gpu buffer will be reallocated to
    /// fit the new instances Vec with 1.5 times extra space.
    pub fn reupload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[D]) {
        if data.len() > self.capacity {
            let new_gpu_buffer_capacity = (data.len() as f32 * 1.5f32) as usize;

            self.buffer.destroy();
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&self.label),
                size: (std::mem::size_of::<D>() * new_gpu_buffer_capacity) as u64,
                usage: self.usage,
                mapped_at_creation: false,
            });

            self.capacity = new_gpu_buffer_capacity;
        }

        let data_bytes = unsafe { util::slice_as_u8_slice(data) };
        self.len = data.len();

        queue.write_buffer(&self.buffer, 0, data_bytes);
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

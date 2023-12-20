use super::handle::{BufferUsage, GLHandle};

struct ByteBuffer {
    handle: GLHandle,
    size: usize,
}

impl ByteBuffer {
    fn handle(&self) -> &GLHandle {
        &self.handle
    }

    fn bind(&self, usage: BufferUsage) {
        unsafe {
            gl::BindBuffer(usage.into(), self.handle.get());
        }
    }

    fn bind_to(&self, usage: BufferUsage, index: u32) {
        assert!(matches!(usage, BufferUsage::Uniform | BufferUsage::Storage));

        unsafe {
            gl::BindBufferBase(usage.into(), index, self.handle.get());
        }
    }
}

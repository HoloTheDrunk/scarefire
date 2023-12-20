use std::ffi::c_void;

use gl::types::GLuint;

use super::handle::{BufferUsage, GLHandle};

struct ByteBuffer {
    handle: GLHandle,
    size: usize,
}

fn create_buffer_handle() -> GLuint {
    let handle: GLuint = 0;
    unsafe {
        gl::CreateBuffers(1, handle as *mut GLuint);
    }
    handle
}

impl ByteBuffer {
    fn new<T>(data: &[T]) -> Self {
        let handle = GLHandle::new(create_buffer_handle());
        let size = data.len();

        unsafe {
            gl::NamedBufferData(
                handle.get(),
                size as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        Self { handle, size }
    }

    fn handle(&self) -> &GLHandle {
        &self.handle
    }

    fn size(&self) -> usize {
        self.size
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

impl Drop for ByteBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, self.handle.get() as *const u32) }
    }
}

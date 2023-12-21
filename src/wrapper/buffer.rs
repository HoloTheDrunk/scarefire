use std::ffi::c_void;

use gl::types::GLuint;

use super::handle::{BufferUsage, GLHandle};

pub struct GLBuffer<T> {
    handle: GLHandle,
    size: usize,
    marker: std::marker::PhantomData<T>, // CURSED
}

fn create_buffer_handle() -> GLuint {
    let handle: GLuint = 0;
    unsafe {
        gl::CreateBuffers(1, handle as *mut GLuint);
    }
    handle
}

impl<T> GLBuffer<T> {
    pub fn new(data: &[T]) -> Self {
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

        Self {
            handle,
            size,
            marker: std::marker::PhantomData::<T>, // CURSED
        }
    }

    pub fn handle(&self) -> &GLHandle {
        &self.handle
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn bind(&self, usage: BufferUsage) {
        unsafe {
            gl::BindBuffer(usage.into(), self.handle.get());
        }
    }

    pub fn bind_to(&self, usage: BufferUsage, index: u32) {
        assert!(matches!(usage, BufferUsage::Uniform | BufferUsage::Storage));

        unsafe {
            gl::BindBufferBase(usage.into(), index, self.handle.get());
        }
    }
}

impl<T> Drop for GLBuffer<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, self.handle.get() as *const u32) }
    }
}

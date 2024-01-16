use std::ffi::c_void;

use gl::types::GLuint;

use crate::{prelude::dogl, AsSlice};

use super::handle::{BufferUsage, GLHandle};

/// Representation for a buffer stored on the GPU.
pub struct GLBuffer<T> {
    /// GPU buffer handle.
    handle: GLHandle,
    /// Number of elements in the buffer.
    size: usize,
    /// Marker for the GPU buffer data type.
    r#type: std::marker::PhantomData<T>,
}

impl<T> GLBuffer<T> {
    pub fn new(data: &[T]) -> Self {
        fn create_buffer_handle() -> GLHandle {
            let mut handle: GLuint = 0;
            unsafe {
                dogl!(gl::CreateBuffers(1, &mut handle as *mut GLuint));
            }
            GLHandle::new(handle)
        }

        unsafe {
            let data = data.as_u8_slice();
            let handle = create_buffer_handle();
            let size = data.len();

            dogl!(gl::NamedBufferData(
                handle.get(),
                // Byte size
                (size * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            ));

            Self {
                handle,
                size,
                r#type: std::marker::PhantomData::<T>,
            }
        }
    }

    pub fn handle(&self) -> &GLHandle {
        &self.handle
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn byte_size(&self) -> usize {
        self.size * std::mem::size_of::<T>()
    }

    pub fn bind(&self, usage: BufferUsage) {
        unsafe {
            dogl!(gl::BindBuffer(usage.into(), self.handle.get()));
        }
    }

    pub fn bind_to(&self, usage: BufferUsage, index: u32) {
        assert!(matches!(usage, BufferUsage::Uniform | BufferUsage::Storage));

        unsafe {
            dogl!(gl::BindBufferBase(usage.into(), index, self.handle.get()));
        }
    }
}

impl<T> Drop for GLBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            dogl!(gl::DeleteBuffers(1, &self.handle.get() as *const u32));
        }
    }
}

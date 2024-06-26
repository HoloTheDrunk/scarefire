use gl::types::GLuint;

pub struct GLHandle {
    handle: GLuint,
}

impl GLHandle {
    pub fn get(&self) -> GLuint {
        self.handle
    }

    pub fn is_valid(&self) -> bool {
        self.handle > 0
    }

    pub fn new(handle: GLuint) -> Self {
        Self { handle }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUsage {
    Attribute,
    Index,
    Uniform,
    Storage,
}

impl From<BufferUsage> for gl::types::GLenum {
    fn from(value: BufferUsage) -> Self {
        match value {
            BufferUsage::Attribute => gl::ARRAY_BUFFER,
            BufferUsage::Index => gl::ELEMENT_ARRAY_BUFFER,
            BufferUsage::Uniform => gl::UNIFORM_BUFFER,
            BufferUsage::Storage => gl::SHADER_STORAGE_BUFFER,
        }
    }
}

pub enum AccessType {
    WriteOnly,
    ReadOnly,
    ReadWrite,
}

impl From<AccessType> for gl::types::GLenum {
    fn from(value: AccessType) -> Self {
        match value {
            AccessType::WriteOnly => gl::WRITE_ONLY,
            AccessType::ReadOnly => gl::READ_ONLY,
            AccessType::ReadWrite => gl::READ_WRITE,
        }
    }
}

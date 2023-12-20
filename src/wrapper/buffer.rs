pub struct GLHandle {
    handle: u32,
}

impl GLHandle {
    pub fn get(&self) -> u32 {
        self.handle
    }

    pub fn is_valid(&self) -> bool {
        self.handle > 0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Usage {
    Attribute,
    Index,
    Uniform,
    Storage,
}

impl From<Usage> for gl::types::GLenum {
    fn from(value: Usage) -> Self {
        match value {
            Usage::Attribute => gl::ARRAY_BUFFER,
            Usage::Index => gl::ELEMENT_ARRAY_BUFFER,
            Usage::Uniform => gl::UNIFORM_BUFFER,
            Usage::Storage => gl::SHADER_STORAGE_BUFFER,
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

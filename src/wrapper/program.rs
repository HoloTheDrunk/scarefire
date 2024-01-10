use std::cmp::Ordering;

use crate::{handle::GLHandle, hash::str_hash};
use paste::paste;

struct UniformLocationInfo {
    hash: u32,
    location: u32,
}

impl PartialEq for UniformLocationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl PartialOrd for UniformLocationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

macro_rules! set_uniform {
    ($typename: ty => $expr: expr) => {
        paste! {
            pub fn [<set_uniform_ $typename>](&self, name_hash: u32, value: $typename) {
                let loc = self.find_location(name_hash);
                if let Some(loc) = loc {
                    $expr(self, value, loc as i32);
                }
            }
        }
    };
}

pub struct Program {
    handle: GLHandle,
    uniform_locations: Vec<UniformLocationInfo>,
    is_compute: bool,
}

impl Program {
    pub fn new_shader() -> Self {
        Self {
            handle: todo!(),
            uniform_locations: todo!(),
            is_compute: todo!(),
        }
    }

    pub fn new_compute_shader() -> Self {
        Self {
            handle: todo!(),
            uniform_locations: todo!(),
            is_compute: todo!(),
        }
    }

    set_uniform! {
        u32 => |program: &Program, value, loc| unsafe {
            gl::ProgramUniform1ui(program.handle.get(), loc, value);
        }
    }

    fn find_location(&self, hash: u32) -> Option<u32> {
        self.uniform_locations
            .iter()
            .find(|e| e.hash == hash)
            .map(|e| e.location)
    }
}

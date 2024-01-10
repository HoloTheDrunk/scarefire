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
    ($($typename: path as $fnc_name: ident => $expr: expr),+$(,)?) => {
        $(paste! {
            pub fn [<set_uniform_ $fnc_name>](&self, name_hash: u32, value: $typename) {
                let loc = self.find_location(name_hash);
                if let Some(loc) = loc {
                    $expr(self, value, loc as i32);
                }
            }
        })+
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
        u32 as u32 => |program: &Program, value, loc| unsafe {
            gl::ProgramUniform1ui(program.handle.get(), loc, value);
        },

        f32 as f32 => |program: &Program, value, loc| unsafe {
            gl::ProgramUniform1f(program.handle.get(), loc, value);
        },

        glm::Vec2 as vec2 => |program: &Program, value: glm::Vec2, loc| unsafe {
            gl::ProgramUniform2f(program.handle.get(), loc, value.x, value.y);
        },

        glm::Vec3 as vec3 => |program: &Program, value: glm::Vec3, loc| unsafe {
            gl::ProgramUniform3f(program.handle.get(), loc, value.x, value.y, value.z);
        },

        glm::Vec4 as vec4 => |program: &Program, value: glm::Vec4, loc| unsafe {
            gl::ProgramUniform4f(program.handle.get(), loc, value.x, value.y, value.z, value.w);
        },

        // FIXME: does not work lol
        // glm::Mat2 as mat2 => |program: &Program, value: glm::Mat2, loc| unsafe {
        //     gl::ProgramUniformMatrix2fv(program.handle.get(), loc, 1, false, &value);
        // },

        // glm::Mat3 as mat3 => |program: &Program, value: glm::Vec3, loc| unsafe {
        // },

        // glm::Mat4 as mat4 =>|program: &Program, value: glm::Vec3, loc| unsafe {
        // },
    }

    fn find_location(&self, hash: u32) -> Option<u32> {
        self.uniform_locations
            .iter()
            .find(|e| e.hash == hash)
            .map(|e| e.location)
    }
}

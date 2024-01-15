use std::cmp::Ordering;

use crate::{handle::GLHandle, hash::str_hash};
use glam::Vec2;
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

impl Eq for UniformLocationInfo {}

impl PartialOrd for UniformLocationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for UniformLocationInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}

macro_rules! set_uniform {
    () => {};

    (@inner $self:ident, $name_hash:ident, $value:ident, $expr:expr) => {
        if let Some(loc) = $self.find_location($name_hash) {
            $expr($self.handle.get(), $value, loc as i32);
        }
    };

    ($typename: path as $fnc_name: ident => $expr: expr $(, $($tail:tt)*)?) => {
        paste! {
            pub fn [<set_uniform_ $fnc_name>](&self, name_hash: u32, value: $typename) {
                set_uniform!(@inner self, name_hash, value, $expr);
            }

            $(set_uniform!($($tail)*);)?
        }
    };

    (&$typename: path as $fnc_name: ident => $expr: expr $(, $($tail:tt)*)?) => {
        paste! {
            pub fn [<set_uniform_ $fnc_name>](&self, name_hash: u32, value: &$typename) {
                set_uniform!(@inner self, name_hash, value, $expr);
            }

            $(set_uniform!($($tail)*);)?
        }
    };
}

pub struct ShaderPaths {
    vertex: String,
    geometry: Option<String>,
    tess_control: Option<String>,
    tess_evaluation: Option<String>,
    fragment: String,
}

pub struct Program {
    handle: GLHandle,
    uniform_locations: Vec<UniformLocationInfo>,
    is_compute: bool,
}

impl Program {
    pub fn new_shader(paths: ShaderPaths) -> Self {
        unsafe {
            let handle = gl::CreateProgram();

            let vert_handle = load_shader(paths.vertex.as_ref(), gl::VERTEX_SHADER);
            let geometry_handle = paths
                .geometry
                .map(|path| load_shader(path.as_ref(), gl::GEOMETRY_SHADER));
            let tess_control_handle = paths
                .tess_control
                .map(|path| load_shader(path.as_ref(), gl::TESS_CONTROL_SHADER));
            let tess_eval_handle = paths
                .tess_evaluation
                .map(|path| load_shader(path.as_ref(), gl::TESS_EVALUATION_SHADER));
            let frag_handle = load_shader(paths.fragment.as_ref(), gl::FRAGMENT_SHADER);

            let shader_handles = [
                Some(vert_handle),
                geometry_handle,
                tess_control_handle,
                tess_eval_handle,
                Some(frag_handle),
            ];

            for shader_handle in shader_handles.iter().flatten() {
                gl::AttachShader(handle, *shader_handle);
            }

            gl::LinkProgram(handle);
            check_program_error(handle);

            for shader_handle in shader_handles.iter().flatten() {
                gl::DeleteShader(*shader_handle);
            }

            Self {
                handle: GLHandle::new(handle),
                uniform_locations: Program::fetch_uniform_locations(handle),
                is_compute: false,
            }
        }
    }

    pub fn new_compute_shader(comp: &str) -> Self {
        unsafe {
            let handle = gl::CreateProgram();
            let comp_shader = load_shader(comp, gl::COMPUTE_SHADER);

            gl::AttachShader(handle, comp_shader);

            gl::LinkProgram(handle);
            check_program_error(handle);

            gl::DeleteShader(comp_shader);

            Self {
                handle: GLHandle::new(handle),
                uniform_locations: Program::fetch_uniform_locations(handle),
                is_compute: true,
            }
        }
    }

    set_uniform! {
        u32 as u32 => |handle, value, loc| unsafe {
            gl::ProgramUniform1ui(handle, loc, value);
        },

        f32 as f32 => |handle, value, loc| unsafe {
            gl::ProgramUniform1f(handle, loc, value);
        },

        glam::Vec2 as vec2 => |handle, value: glam::Vec2, loc| unsafe {
            gl::ProgramUniform2f(handle, loc, value.x, value.y);
        },

        glam::Vec3 as vec3 => |handle, value: glam::Vec3, loc| unsafe {
            gl::ProgramUniform3f(handle, loc, value.x, value.y, value.z);
        },

        glam::Vec4 as vec4 => |handle, value: glam::Vec4, loc| unsafe {
            gl::ProgramUniform4f(handle, loc, value.x, value.y, value.z, value.w);
        },

        &glam::Mat2 as mat2 => |handle, value: &glam::Mat2, loc| unsafe {
            gl::ProgramUniformMatrix2fv(handle, loc, 1, gl::FALSE, value as *const glam::Mat2 as *const f32);
        },

        &glam::Mat3 as mat3 => |handle, value: &glam::Mat3, loc| unsafe {
            gl::ProgramUniformMatrix3fv(handle, loc, 1, gl::FALSE, value as *const glam::Mat3 as *const f32);
        },

        &glam::Mat4 as mat4 => |handle, value: &glam::Mat4, loc| unsafe {
            gl::ProgramUniformMatrix4fv(handle, loc, 1, gl::FALSE, value as *const glam::Mat4 as *const f32);
        },
    }

    fn find_location(&self, hash: u32) -> Option<u32> {
        self.uniform_locations
            .iter()
            .find(|e| e.hash == hash)
            .map(|e| e.location)
    }

    fn fetch_uniform_locations(handle: u32) -> Vec<UniformLocationInfo> {
        let mut uniform_locations = Vec::new();

        unsafe {
            let mut uniform_count = 0;
            gl::GetProgramiv(handle, gl::ACTIVE_UNIFORMS, &mut uniform_count);

            for i in 0..uniform_count {
                let mut name = [0u8; 1024];
                let mut len = 0i32;
                let mut discard = 0i32;
                let mut type_ = gl::NONE;
                gl::GetActiveUniform(
                    handle,
                    i as gl::types::GLuint,
                    1024,
                    &mut len,
                    &mut discard,
                    &mut type_,
                    &mut name as *mut u8 as *mut i8,
                );

                uniform_locations.push(UniformLocationInfo {
                    hash: str_hash(
                        name.into_iter()
                            .map(char::from)
                            .collect::<String>()
                            .as_ref(),
                    ),
                    location: gl::GetUniformLocation(handle, &name as *const u8 as *const i8)
                        as u32,
                });
            }

            uniform_locations.sort();
            assert!(!uniform_locations.windows(2).any(|arr| arr[0] == arr[1]));

            uniform_locations
        }
    }
}

fn load_shader(path: &str, r#type: gl::types::GLenum) -> u32 {
    let code = std::fs::read_to_string(path).expect(format!("Couldn't open {path:?}").as_str());

    unsafe {
        let shader = gl::CreateShader(r#type);
        assert_ne!(shader, 0);

        compile_shader(shader, code.as_str());

        check_shader_error(
            shader,
            match r#type {
                gl::VERTEX_SHADER => "Vertex",
                gl::TESS_CONTROL_SHADER => "Tesselation Control",
                gl::TESS_EVALUATION_SHADER => "Tesselation Evaluation",
                gl::GEOMETRY_SHADER => "Geometry",
                gl::FRAGMENT_SHADER => "Fragment",
                gl::COMPUTE_SHADER => "Compute",
                _ => panic!("Unsupported shader type"),
            },
        );

        shader
    }
}

fn compile_shader(shader: u32, shader_code: &str) {
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &(shader_code.as_bytes().as_ptr().cast()),
            &(shader_code.len().try_into().unwrap()),
        );

        gl::CompileShader(shader);
    }
}

fn check_shader_error(shader: u32, shader_type: &str) {
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut vec = Vec::<u8>::with_capacity(log_length as usize);
            let mut returned_log_length = 0;
            gl::GetShaderInfoLog(
                shader,
                log_length,
                &mut returned_log_length,
                vec.as_mut_ptr().cast(),
            );

            vec.set_len(returned_log_length.try_into().unwrap());

            panic!(
                "{shader_type} compile error: {}",
                String::from_utf8_lossy(&vec)
            )
        }
    }
}

fn check_program_error(program: u32) {
    unsafe {
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut vec = Vec::<u8>::with_capacity(log_length as usize);
            let mut returned_log_length = 0;
            gl::GetProgramInfoLog(
                program,
                log_length,
                &mut returned_log_length,
                vec.as_mut_ptr().cast(),
            );

            vec.set_len(returned_log_length.try_into().unwrap());

            panic!(
                "Program link error: {} ({vec:?})",
                String::from_utf8_lossy(&vec)
            )
        }
    }
}

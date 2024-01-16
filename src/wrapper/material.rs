use std::{collections::HashMap, rc::Rc};

use super::{program::Program, texture::Texture};

pub enum BlendMode {
    None,
    Alpha,
}

pub enum DepthTestMode {
    None,
    Equal,
    Standard,
    Reversed,
}

pub struct Material {
    pub program: Rc<Program>,
    textures: HashMap<u32, Rc<Texture>>,

    pub blend_mode: BlendMode,
    pub depth_test_mode: DepthTestMode,
}

impl Material {
    pub fn new(program: Rc<Program>) -> Self {
        Self {
            program,
            textures: HashMap::new(),
            blend_mode: BlendMode::None,
            depth_test_mode: DepthTestMode::Standard,
        }
    }
    pub fn set_texture(&mut self, slot: u32, texture: &Rc<Texture>) {
        self.textures
            .entry(slot)
            .and_modify(|tex| *tex = texture.clone())
            .or_insert_with(|| texture.clone());
    }

    pub fn bind(&self) {
        unsafe {
            match self.blend_mode {
                BlendMode::None => gl::Disable(gl::BLEND),
                BlendMode::Alpha => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
            }

            match self.depth_test_mode {
                DepthTestMode::None => gl::Disable(gl::DEPTH_TEST),
                DepthTestMode::Equal => {
                    gl::Enable(gl::DEPTH_TEST);
                    gl::DepthFunc(gl::EQUAL);
                }
                DepthTestMode::Standard => {
                    gl::Enable(gl::DEPTH_TEST);
                    gl::DepthFunc(gl::GEQUAL);
                }
                DepthTestMode::Reversed => {
                    gl::Enable(gl::DEPTH_TEST);
                    gl::DepthFunc(gl::LEQUAL);
                }
            }

            for (handle, texture) in self.textures.iter() {
                texture.bind(*handle);
            }

            self.program.bind();
        }
    }
}

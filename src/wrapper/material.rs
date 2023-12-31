use std::rc::Rc;

use super::{program::Program, texture::Texture};

enum BlendMode {
    None,
    Alpha,
}

enum DepthTestMode {
    Standard,
    Reversed,
    Equal,
}

pub struct Material {
    program: Rc<Program>,
    textures: Vec<(u32, Rc<Texture>)>,

    blend_mode: BlendMode,
    depth_test_mode: DepthTestMode,
}

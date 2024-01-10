use stb_image::stb_image::stbi_load;

use crate::handle::{AccessType, GLHandle};

use {
    gl::types::{GLenum, GLuint},
    glm::GenNumVec,
};

#[derive(Clone, Copy)]
enum ImageFormat {
    Rgba8Unorm,
    Rgba8SRgb,

    Rgb8Unorm,
    Rgb8SRgb,

    Rgba16Float,

    Depth32Float,
}

impl ImageFormat {
    fn to_gl(&self) -> ImageFormatGL {
        ImageFormatGL::from(*self)
    }
}

struct ImageFormatGL {
    format: GLenum,
    internal_format: GLenum,
    component_type: GLenum,
}

impl From<ImageFormat> for ImageFormatGL {
    fn from(format: ImageFormat) -> Self {
        let (format, internal_format, component_type) = match format {
            ImageFormat::Rgba8Unorm => (gl::RGBA, gl::RGBA8, gl::UNSIGNED_BYTE),
            ImageFormat::Rgba8SRgb => (gl::RGBA, gl::SRGB8_ALPHA8, gl::UNSIGNED_BYTE),

            ImageFormat::Rgb8Unorm => (gl::RGB, gl::RGB8, gl::UNSIGNED_BYTE),
            ImageFormat::Rgb8SRgb => (gl::RGB, gl::SRGB8, gl::UNSIGNED_BYTE),

            ImageFormat::Rgba16Float => (gl::RGBA, gl::RGBA16F, gl::FLOAT),

            ImageFormat::Depth32Float => (gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT32F, gl::FLOAT),
        };

        ImageFormatGL {
            format,
            internal_format,
            component_type,
        }
    }
}

pub struct TextureData {
    data: Vec<u8>,
    size: glm::UVec2,
    format: ImageFormat
}

impl TextureData {
    pub fn from_file(path: &str) -> Option<TextureData> {
        let mut width = 0;
        let mut height = 0;
        let mut channels = 0;

        let bytes = unsafe {
            let img = stbi_load(path.as_ptr() as * const std::os::raw::c_char, &mut width, &mut height, &mut channels, 4);
            std::slice::from_raw_parts(img, (width * height * channels) as usize).to_vec()
        };


        if (width <= 0 || height <= 0 || channels <= 0) {
            return None;
        }

        Some(TextureData {
            data: bytes,
            size: glm::uvec2(width as u32, height as u32),
            format: ImageFormat::Rgba8Unorm,
        })
    }
}

pub struct Texture {
    handle: GLHandle,
    size: glm::UVec2,
    format: ImageFormat,
}

impl Texture {
    pub fn new(data : &TextureData) -> Self {
        Self {
            handle: todo!(),
            size: todo!(),
            format: todo!(),
        }
    }
    pub fn bind(&self, index: GLuint) {
        unsafe {
            gl::BindTextureUnit(index, self.handle.get());
        }
    }

    pub fn bind_as_image(&self, index: GLuint, access: AccessType) {
        unsafe {
            gl::BindImageTexture(
                index,
                self.handle.get(),
                0,
                false as gl::types::GLboolean,
                0,
                access.into(),
                self.format.to_gl().internal_format,
            );
        }
    }

    pub fn size(&self) -> &glm::UVec2 {
        &self.size
    }

    /// Return number of mip levels needed.
    pub fn mip_levels(size: glm::UVec2) -> u32 {
        let side = size.max() as f32;
        1 + side.log2().floor() as u32
    }
}

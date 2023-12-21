use super::{buffer::GLBuffer, handle::BufferUsage};

struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub color: glm::Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: glm::vec3(0., 0., 0.),
            normal: glm::vec3(0., 1., 0.),
            color: glm::vec3(1., 1., 1.),
        }
    }
}

struct StaticMesh {
    vertex_buffer: GLBuffer<Vertex>,
    index_buffer: GLBuffer<u32>,
}

impl StaticMesh {
    pub fn draw(&self) {
        self.vertex_buffer.bind(BufferUsage::Attribute);
        self.index_buffer.bind(BufferUsage::Index);

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                std::ptr::null(),
            );

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (3 * std::mem::size_of::<f32>()) as *mut std::ffi::c_void,
            )
        };
    }
}

struct SceneObject {
    transform: glm::Mat4,
}

struct Scene {}

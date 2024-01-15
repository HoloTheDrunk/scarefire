use super::{buffer::GLBuffer, handle::BufferUsage};

#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub color: glam::Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: glam::vec3(0., 0., 0.),
            normal: glam::vec3(0., 1., 0.),
            color: glam::vec3(1., 1., 1.),
        }
    }
}

struct Range {
    min: f32,
    max: f32,
}

impl Range {
    fn stretch_to(&mut self, v: f32) {
        if v < self.min {
            self.min = v;
        } else if v > self.max {
            self.max = v;
        }
    }
}

impl From<f32> for Range {
    fn from(value: f32) -> Self {
        Self {
            min: value,
            max: value,
        }
    }
}

struct Bounds {
    x: Range,
    y: Range,
    z: Range,
}

impl Bounds {
    pub fn new(mut vertices: impl Iterator<Item = glam::Vec3>) -> Self {
        let first = vertices
            .next()
            .expect("Cannot create bounds from empty vertices vector");

        let mut res = Self::from(first);

        for v in vertices {
            res.stretch_to(v);
        }

        res
    }

    pub fn stretch_to(&mut self, pos: glam::Vec3) {
        self.x.stretch_to(pos.x);
        self.y.stretch_to(pos.y);
        self.z.stretch_to(pos.z);
    }

    pub fn get_center(&self) -> glam::Vec3 {
        glam::Vec3 {
            x: (self.x.min + self.x.max) / 2.,
            y: (self.y.min + self.y.max) / 2.,
            z: (self.z.min + self.z.max) / 2.,
        }
    }

    pub fn get_min(&self) -> glam::Vec3 {
        glam::vec3(self.x.min, self.y.min, self.z.min)
    }

    pub fn get_max(&self) -> glam::Vec3 {
        glam::vec3(self.x.max, self.y.max, self.z.max)
    }
}

impl From<glam::Vec3> for Bounds {
    fn from(value: glam::Vec3) -> Self {
        Self {
            x: Range::from(value.x),
            y: Range::from(value.y),
            z: Range::from(value.z),
        }
    }
}

pub struct BoundingSphere {
    pub center: glam::Vec3,
    pub radius: f32,
}

pub struct StaticMesh {
    vertex_buffer: GLBuffer<Vertex>,
    index_buffer: GLBuffer<u32>,
    bounding_sphere: BoundingSphere,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
    ::core::slice::from_raw_parts((p.as_ptr()) as *const u8, ::core::mem::size_of::<T>())
}

impl StaticMesh {
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        let bounds = Bounds::new(vertices.iter().map(|v| v.position));
        let center = bounds.get_center();

        unsafe {
            Self {
                vertex_buffer: GLBuffer::new(any_as_u8_slice(vertices)),
                index_buffer: GLBuffer::new(any_as_u8_slice(indices)),
                bounding_sphere: BoundingSphere {
                    center,
                    radius: bounds
                        .get_min()
                        .distance(center)
                        .max(bounds.get_max().distance(center)),
                },
            }
        }
    }

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

    pub fn bounds(&self) -> &BoundingSphere {
        &self.bounding_sphere
    }
}

use crate::{camera::Camera, material::Material, mesh::StaticMesh};

use std::rc::Rc;

pub struct TransformComponents {
    // ==== Regular ones
    position: glm::Vec3,
    // TODO: PR glm-rs with quat impl
    rotation: glm::Vec4,
    scale: glm::Vec3,

    // ==== Funky ones
    skew: glm::Vec3,
    perspective: glm::Vec4,
}

pub struct SceneObject {
    pub transform: glm::Mat4,
    mesh: Rc<StaticMesh>,
    material: Rc<Material>,
}

impl SceneObject {
    fn render(&self) {
        todo!()
    }

    pub fn mesh(&self) -> &Rc<StaticMesh> {
        &self.mesh
    }

    pub fn material(&self) -> &Rc<Material> {
        &self.material
    }

    pub fn get_transform_components(&self) -> TransformComponents {
        let (scale, rotation, position, skew, perspective) = self
            .transform
            .decompose()
            .expect("Failed to decompose mat4");

        TransformComponents {
            position,
            rotation,
            scale,
            skew,
            perspective,
        }
    }
}

struct Scene {
    objects: Vec<SceneObject>,
    // point_ligts: Vec<PointLight>,
    sun_direction: glm::Vec3,
    sun_color: glm::Vec3,

    camera: Camera,
}

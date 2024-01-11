use crate::{camera::Camera, material::Material, mesh::StaticMesh};

use std::rc::{Rc, Weak};

use weak_table::PtrWeakKeyHashMap as WeakMap;

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

struct ObjectStorage {
    inner: WeakMap<Weak<Material>, WeakMap<Weak<StaticMesh>, Vec<SceneObject>>>,
}

impl ObjectStorage {
    fn insert(&mut self, object: SceneObject) {
        self.inner
            .entry(object.material.clone())
            .or_insert_with(WeakMap::new)
            .entry(object.mesh.clone())
            .or_insert_with(Vec::new)
            .push(object);
    }

    fn iter_groups(&self) -> impl Iterator<Item = &Vec<SceneObject>> {
        self.inner.values().flat_map(|map| map.values())
    }

    fn len(&self) -> usize {
        self.inner
            .values()
            .flat_map(WeakMap::values)
            .fold(0, |acc, vec| acc + vec.len())
    }
}

pub struct Scene {
    // point_ligts: Vec<PointLight>,
    objects: ObjectStorage,

    pub sun_direction: glm::Vec3,
    pub sun_color: glm::Vec3,

    pub camera: Camera,
}

impl Scene {
    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.insert(object);
    }

    pub fn render(&self) {
        todo!()
    }
}

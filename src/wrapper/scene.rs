use crate::{buffer::GLBuffer, camera::Camera, material::Material, mesh::StaticMesh, AsSlice};

use glrs::import;

use std::rc::{Rc, Weak};

use weak_table::PtrWeakKeyHashMap as WeakMap;

pub struct TransformComponents {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

pub struct SceneObject {
    pub transform: glam::Mat4,
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
        let (scale, rotation, position) = self.transform.to_scale_rotation_translation();

        TransformComponents {
            position,
            rotation,
            scale,
        }
    }
}

#[derive(Default)]
pub struct ObjectStorage {
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

    fn iter_material_groups(&self) -> impl Iterator<Item = &Vec<SceneObject>> {
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
    pub objects: ObjectStorage,

    pub sun_direction: glam::Vec3,
    pub sun_color: glam::Vec3,

    pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            sun_direction: glam::vec3(1., 1., 1.),
            sun_color: glam::vec3(1., 1., 1.),
            camera: Default::default(),
        }
    }
}

#[glrs::import(path = "shaders/structs.glsl")]
#[derive(Default)]
struct FrameData;

#[glrs::import(path = "shaders/structs.glsl")]
struct PointLight;

impl Scene {
    pub fn new(sun_direction: glam::Vec3, sun_color: glam::Vec3, camera: Camera) -> Self {
        Self {
            objects: ObjectStorage::default(),
            sun_direction,
            sun_color,
            camera,
        }
    }

    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.insert(object);
    }

    pub fn render(&self) {
        let frame_data = FrameData {
            view_proj: self.camera.view_proj,
            sun_dir: self.sun_direction.normalize(),
            sun_color: self.sun_color,
            ..Default::default()
        };
        let buffer = unsafe { GLBuffer::new(&[frame_data]) };

        // TODO: Batch rendering
        for vec in self.objects.iter_material_groups() {
            vec[0].material().bind();
            for object in vec.iter() {
                object.render();
            }
        }
    }
}

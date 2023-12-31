use std::rc::Rc;

use super::mesh::StaticMesh;

struct SceneObject {
    transform: glm::Mat4,
    mesh: Rc<StaticMesh>,
    // TODO
    // material: Rc<Material>,
}

struct Scene {}

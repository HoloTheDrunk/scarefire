use std::rc::Rc;

use glam::vec3;
use glfw::{Action, Context, Key};
use scarefire::prelude::*;

fn init_scene() -> Scene {
    let mut scene = Scene::default();

    let vertices = [
        vec3(-1., -1., 0.),
        vec3(-1., 1., 0.),
        vec3(1., 1., 0.),
        vec3(1., -1., 0.),
    ]
    .into_iter()
    .map(|position| Vertex {
        position,
        normal: position / 2. + 1.,
        color: vec3(1., 1., 1.),
    })
    .collect::<Vec<_>>();

    let indices = [0, 1, 2, 2, 3, 0];

    let mesh = StaticMesh::new(vertices.as_slice(), &indices);

    let program = Program::new_shader(ShaderPaths {
        fragment: "shaders/plane/fragment.glsl".to_owned(),
        geometry: None,
        tess_control: None,
        tess_evaluation: None,
        vertex: "shaders/plane/vertex.glsl".to_owned(),
    });

    let material = Material::new(Rc::new(program));

    let obj = SceneObject::new(Rc::new(mesh), Rc::new(material));

    scene.add_object(obj);

    scene
}

fn main() {
    println!("OooOooOOoOOOoo");

    let (mut glfw, (mut window, events)) = unsafe { init_graphics() };

    let scene = init_scene();

    // Loop until the user closes the window
    while !window.should_close() {
        scene.render();

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

use glfw::{Action, Context, Key};
use scarefire::init_graphics;

fn main() {
    println!("OooOooOOoOOOoo");

    let (mut glfw, (mut window, events)) = unsafe { init_graphics() };

    // Loop until the user closes the window
    while !window.should_close() {
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

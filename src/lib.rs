mod wrapper;

use std::{ffi::c_void, ptr::null};

use {
    gl::{
        load_with,
        types::{GLchar, GLenum, GLsizei, GLuint},
    },
    glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent},
};

pub static GLOBAL_VAO: GLuint = 0;

extern "system" fn debug_out(
    _source: GLenum,
    gltype: GLenum,
    _id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
        return;
    }

    let is_error: bool = gltype == gl::DEBUG_TYPE_ERROR;

    let string = format!(
        "[GL]{} {}",
        if severity == gl::DEBUG_SEVERITY_HIGH {
            "[HIGH]"
        } else {
            ""
        },
        String::from_utf8(
            unsafe { std::slice::from_raw_parts(message, length as usize) }
                .into_iter()
                .map(|&v| v as u8)
                .collect::<Vec<u8>>()
        )
        .unwrap()
    );

    if is_error {
        eprintln!("{string}");
    } else {
        println!("{string}");
    };
}

pub unsafe fn init_graphics() -> (Glfw, (PWindow, GlfwReceiver<(f64, WindowEvent)>)) {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    load_with(|f_name| window.get_proc_address(f_name));

    println!(
        "OpenGL {} initialized on {} {} using GLSL {}",
        *gl::GetString(gl::VERSION),
        *gl::GetString(gl::VENDOR),
        *gl::GetString(gl::RENDERER),
        *gl::GetString(gl::SHADING_LANGUAGE_VERSION)
    );

    gl::ClearColor(0.5, 0.7, 0.8, 0.);

    gl::DebugMessageCallback(Some(debug_out), null());
    gl::Enable(gl::DEBUG_OUTPUT);

    gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
    gl::ClearDepth(0.);

    gl::ActiveTexture(gl::TEXTURE0);
    gl::Enable(gl::FRAMEBUFFER_SRGB);

    gl::GenVertexArrays(1, GLOBAL_VAO as *mut GLuint);
    gl::BindVertexArray(GLOBAL_VAO);

    (glfw, (window, events))
}

extern crate gl;
extern crate glfw;

use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, _) = glfw.create_window(800, 600, "OpenGL Fundamentals", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

dgdgsg
    
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}


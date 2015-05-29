extern crate gl;
extern crate glfw;

mod shader_loader;

use std::mem;
use std::ptr;
use shader_loader::*;
use gl::types::*;
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode};

static VS_SRC: &'static str =
   "#version 330\n\
    layout (location = 0) in vec2 position;\
    void main() {\
       gl_Position = vec4(position.xy, 0.0, 1.0);\
    }";

static FS_SRC: &'static str =
   "#version 330\n\
    out vec4 out_color;\
    void main() {\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\
    }";

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, _) = glfw.create_window(800, 600, "Chapter 1", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));
    
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);
        gl::UseProgram(program);

        let data : [GLfloat; 6] = [-0.8, 0.5,
                                   0.6, 0.0,
                                   -0.8, -0.5];

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       size_of_array(&data),
                       mem::transmute(&data[0]),
                       gl::STATIC_DRAW);

        gl::EnableVertexAttribArray(0 as GLuint);
        gl::VertexAttribPointer(0 as GLuint, 2,
                                gl::FLOAT,
                                gl::FALSE as GLboolean,
                                0,
                                ptr::null());
    }

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
    }
}

fn size_of_array<T>(array: &[T]) -> GLsizeiptr {
    (array.len() * mem::size_of::<T>()) as GLsizeiptr
}


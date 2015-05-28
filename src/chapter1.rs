extern crate gl;
extern crate glfw;

mod shader_loader;

use std::mem;
use std::ptr;
use shader_loader::*;
use gl::types::*;
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode};
use std::ffi::CString;

static VS_SRC: &'static str =
   "#version 330\n\
    layout (location = 0) in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position.xy, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 330\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
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

        let data = [-0.8, 0.5,
                    0.6, 0.0,
                    -0.8, -0.5];

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&data[0]),
                       gl::STATIC_DRAW);
        
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);
        gl::UseProgram(program);

        let c_str = CString::new("out_color").unwrap();
        gl::BindFragDataLocation(program, 0, c_str.as_ptr());

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
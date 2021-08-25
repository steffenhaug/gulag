extern crate glfw;
extern crate gl;
#[macro_use]
extern crate lazy_static;

mod shader;
mod vertex;
mod window;
mod util;

use crate::shader::{Shader};
use crate::vertex::{Vertex, VertexArray};
use crate::window::{Window};

#[allow(dead_code)]
struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}


impl Vertex for Vec2 {
    unsafe fn define_vertex_attrib_pointers() {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<f32>() as i32,
            0 as *const std::os::raw::c_void
        );
    }
}

fn main() {
    let mut win = Window::builder()
        .title("Rust OpenGL Test")
        .resolution(1280, 720)
        .build();

    util::enable_debug_callback();
    println!("{}", util::version_string());


    /* Render pipeline configuration */

    // Load shader source code.
    let shader = {
        let vert = include_str!("shader.vert");
        let frag = include_str!("shader.frag");
        Shader::simple(vert, frag)
    };

    /* Uniforms */
    let u_width  = shader.uniform::<u32>("u_width").unwrap();
    let u_height = shader.uniform::<u32>("u_height").unwrap();


    /* Simple way to cover the entire screen */
    let vertices = [ Vec2::new(-1.0, -1.0),
                     Vec2::new( 1.0, -1.0),
                     Vec2::new( 1.0,  1.0),
                     Vec2::new(-1.0,  1.0), ];

    let indices = [ 0, 1, 2,
                    2, 3, 0, ];

    let vao = VertexArray::create(&vertices, &indices);

    while !win.should_close() {
        /* Rendering */
        win.clear();

        let (w, h) = win.size();
        u_width.set(w);
        u_height.set(h);

        win.draw(&vao, &shader);
        win.size();

        /* After rendering, swap buffers */
        win.swap();
    }
}

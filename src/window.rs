use std::sync::Mutex;

use glfw::{
    Context,
    WindowHint,
    OpenGlProfileHint
};

use crate::shader::Shader;
use crate::vertex::{
    Vertex,
    VertexArray
};

/* This is really not ideal. Unfortunately GLFW makes it pretty
 * difficult to create mutltiple windows with a shared instance
 * since it cant be global and mutable without being lazy_static!,
 * and even worse, this makes it possible to create a window on
 * a separate program thread, which is probably undefined behaviour.
 * 
 * I think the answer for this is to use Vulkan.
 */
lazy_static! {
    static ref GLFW: Mutex<glfw::Glfw> = {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        Mutex::new(glfw)
    };

    static ref LOAD_STATUS: Mutex<bool> = Mutex::new(false);
}

/// Responsible for setting up a os window and an OpenGL context.
/// Unfortunately, a window and a valid context is very closely tied
/// in OpenGL, and it is not easy to create a context without also
/// creating a window.
///
/// GLFW also does not make it easy to load the GL function without
/// creating a window, which means the `Window`-structure will also
/// be responsible for loading the GL procedure adresses.
pub struct Window {
    glfw_win: glfw::Window,
    _glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn builder() -> Builder {
        Builder {
            title: None,
            width: 160,
            height: 144,
            resizable: true,
        }
    }

    pub fn clear(&self, ) {
        unsafe {
            let (w, h) = self.size();
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn size(&self) -> (u32, u32) {
        let (w, h) = self.glfw_win.get_framebuffer_size();
        (w as u32, h as u32)
    }

    pub fn draw<V: Vertex>(&self, vertices: &VertexArray<V>, shader: &Shader) {
        unsafe {
            vertices.select();
            shader.select();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn swap(&mut self) {
        self.glfw_win.swap_buffers();
        let mut glfw = GLFW.lock().unwrap();
        glfw.poll_events();
    }

    pub fn should_close(&self) -> bool {
        self.glfw_win.should_close()
    }
}

pub struct Builder {
    title:     Option<String>,
    width:     u32,
    height:    u32,
    resizable: bool,
}

impl Builder {
    pub fn title(mut self, title: &str) -> Builder {
        self.title = Some(String::from(title));
        self
    }

    pub fn resolution(mut self, width: u32, height: u32) -> Builder {
        self.width = width;
        self.height = height;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Builder {
        self.resizable = resizable;
        self
    }

    pub fn build(self) -> Window {
        let mut glfw = GLFW.lock().unwrap();

        glfw.window_hint(WindowHint::Resizable(self.resizable));

        /* Library assumes 4.4 core so this won't change. */
        glfw.window_hint(WindowHint::ContextVersionMajor(4));
        glfw.window_hint(WindowHint::ContextVersionMinor(4));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        let title = self.title.unwrap_or(String::from("GuLag Window"));

        let (mut win, events) = glfw.create_window(
            self.width,
            self.height,
            &title,
            glfw::WindowMode::Windowed
        ).expect("todo: fix window error handling");

        /* Reset the window hints to not interfere with other windows */
        glfw.default_window_hints();

        let mut status = LOAD_STATUS.lock().unwrap();

        if !*status {
            win.make_current();

            /* VSync */
            glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

            gl::load_with(|s| {
                win.get_proc_address(s)
            });

            *status = true;
        }

        Window {
            glfw_win:    win,
            _glfw_events: events,
        }
    }
}

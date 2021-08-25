use std::ffi::CString;

use gl::types::*;

/// The purpose of the Shader struct is to encapsulate loading,
/// compiling and deleting shaders to hide the GL/C types. The
/// struct implements Drop, so shaders are automatically freed
/// once they can no longer be referenced.
pub struct Shader(GLuint);


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}

impl Shader {
    /// Use the shader in the current opengl context.
    /// This is unsafe because it depends on the caller maintaining
    /// the invariant that an opengl context indeed exists, and that
    /// this shader is valid in it.
    pub unsafe fn select(&self) {
        gl::UseProgram(self.0);
    }

    /// Obtain a handle to the given uniform in the shader to upload
    /// data to the GPU. The uniform location might be invalid if for
    /// exmaple the shader compiler has optimized away a uniform, in
    /// which case `None` will be returned.
    ///
    /// The call to this function specifies the type of the uniform,
    /// and from then on the compiler can type check setting the
    /// uniforms value through the handle.
    pub fn uniform<T>(&self, name: &str) -> Option<Uniform<T>> {
        /* Create a CString from the name. */
        let name = CString::new(name).unwrap();

        /* For some reason, the .as_ptr() call needs to be in the fun call. */
        let location = unsafe { gl::GetUniformLocation(self.0, name.as_ptr()) };

        // idea: generate a table of uniforms up front, so we can
        // check the TypeId of T vs. the actual uniform and panic
        // currently, this is not REALLY super safe since this just lets
        // the user type the uniform the way they want, but assuming the
        // user gets the type right when they retrieve the uniform, the
        // rust compiler will check the rest.

        if location == -1 {
            None
        } else {
            let u = Uniform {
                loc:    location,
                shader: self.0,
                marker: std::marker::PhantomData
            };

            Some(u)
        }
    }

    /// Creates a simple program only containing a vertex and fragment shader.
    pub fn simple(vert: impl AsRef<str>, frag: impl AsRef<str>) -> Shader {
        let vs = Shader::compile(gl::VERTEX_SHADER,   vert.as_ref());
        let fs = Shader::compile(gl::FRAGMENT_SHADER, frag.as_ref());

        unsafe {
            // Create program handle.
            let program = gl::CreateProgram();

            // Attach the compiled shaders and link.
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::ValidateProgram(program);

            // Now that the program is compiled, clean up.
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            // Wrap the handle.
            Shader(program)
        }
    }

    /// Performs the low-level compilation and returns a handle to the
    /// shader object as a GL uint, which we later need to link.
    fn compile(ty: GLuint, src: &str) -> GLuint {
        /* NOTE: to be perfectly safe, this should panic if no gl context exists */
        // Loading shaders should not really modify the GL state in
        // any way that could make other things behave unexpectedly,
        // so calling this function should always be safe to do, as
        // long as OpenGL is actually initialized.
        unsafe {
            let handle = gl::CreateShader(ty);

            // This can only fail if the provided source has internal zero bytes.
            let src = CString::new(src).unwrap();

            // Attach the shader source code to the handle and compile it.
            gl::ShaderSource(handle, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(handle);

            // Check compilation status.
            let mut is_ok = 0;
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut is_ok);

            if (is_ok as GLboolean) == gl::FALSE {
                // If compilation failed, retrieve the error message.
                let mut len  = 0;
                let mut info = Vec::new();

                // Get the error message length, and resize the buffer.
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
                info.resize(len as usize, 0x0);

                // Get the error message itself.
                gl::GetShaderInfoLog(handle, len, &mut len, info.as_ptr() as *mut GLbyte);
                let info = String::from_utf8(info);
                println!("FAILED TO COMPILE. {:?}", info);

                // If we fail to compile the attached source, we delete the handle.
                gl::DeleteShader(handle);

                // Return Zero (invalid shader handle) to indicate failure.
                // todo: this function can return option<gluint>
                return 0;
            }

            return handle;
        }
    }
}

/// Encapsulates a handle to the location of a uniform assuming a
/// particular type, and provides a method to set the value of the
/// uniform in a type checked way dispatched statically.
pub struct Uniform<T> {
    loc:    GLint,
    shader: GLuint,
    marker: std::marker::PhantomData<T>
}

impl Uniform<u32> {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn set(&self, n: u32) {
        unsafe {
            gl::UseProgram(self.shader);
            gl::Uniform1ui(self.loc, n);
        }
    }
}

impl Uniform<f32> {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn set(&self, x: f32) {
        unsafe {
            gl::UseProgram(self.shader);
            gl::Uniform1f(self.loc, x);
        }
    }
}

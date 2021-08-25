use std::os::raw::{
    c_void,
    c_char
};
use gl::types::*;
use std::ffi::{CStr};

/// Enables printing error messages produced by OpenGL to the console
/// by registering a debug-message callback that prints it.
pub fn enable_debug_callback() {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(print_error), std::ptr::null());
    }
}

// glDebugMessageCallback callback function.
extern "system" fn print_error(
    source: GLenum,
    ty: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void
) {
    if severity > gl::DEBUG_SEVERITY_NOTIFICATION {
        let source   = explain_debug_source(source);
        let ty       = explain_debug_type(ty);
        let severity = explain_debug_severity(severity);
        let message  = wrap_string(message);
        eprintln!("{}: [TYPE: {}, SEVERITY: {} IN {}], {}",
                  id, ty, severity, source, message);
    }
}

fn explain_debug_source(source: GLenum) -> &'static str {
    match source {
        gl::DEBUG_SOURCE_API             => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM   => "WINDOW SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY     => "THIRD PARTY",
        gl::DEBUG_SOURCE_APPLICATION     => "APPLICATION",
        _                                => "UNKNOWN SOURCE"
    }
}

fn explain_debug_type(ty: GLenum) -> &'static str {
    match ty {
        gl::DEBUG_TYPE_ERROR               => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR  => "UNDEFINED BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY         => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE         => "PERFORMANCE",
        gl::DEBUG_TYPE_OTHER               => "OTHER",
        gl::DEBUG_TYPE_MARKER              => "MARKER",
        _                                  => "UNKNOWN TYPE"
    }
}

fn explain_debug_severity(severity: GLenum) -> &'static str {
    match severity {
        gl::DEBUG_SEVERITY_HIGH         => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM       => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW          => "LOW",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _                               => "UNKNOWN SEVERITY"
    }
}

// COPIES a null terminated c string from the opengl api to an
// owned rust string which we can manipulate in a convenient way.
pub fn wrap_string(text: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(text) };

    /* Parse as utf-8 */
    let utf8 = cstr.to_str().unwrap();

    String::from(utf8)
}

/// Returns a `String` copy of the OpenGL version string.
pub fn version_string() -> String {
    let version = unsafe { gl::GetString(gl::VERSION) } as *const c_char;
    wrap_string(version)
}

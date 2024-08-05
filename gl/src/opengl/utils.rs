use std;
use std::ffi::CString;

//a Functions
//fp create_whitespace_cstring_with_len
/// Create a CString of 'len' spaces (with null termination)
pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

//fp check_errors
/// Check for OpenGL errors; return Ok if there are none, else all of
/// the errors as strings in a Vec
pub fn check_errors() -> Result<(), Vec<String>> {
    let mut v = Vec::new();
    loop {
        match unsafe { gl::GetError() } {
            gl::NO_ERROR => {
                break;
            }
            gl::INVALID_ENUM => {
                v.push("Invalid enum (Gl error)".to_string());
            }
            gl::INVALID_VALUE => {
                v.push("Invalid value (Gl error)".to_string());
            }
            gl::INVALID_OPERATION => {
                v.push("Invalid operation (Gl error)".to_string());
            }
            x => {
                v.push(format!("GL had error {}", x));
            }
        }
    }
    if v.is_empty() {
        Ok(())
    } else {
        Err(v)
    }
}

//fp get_shaderiv
/// Get an integer value from a particular shader
pub fn get_shaderiv(id: gl::types::GLuint, x: gl::types::GLuint) -> gl::types::GLint {
    unsafe {
        let mut r = 0;
        gl::GetShaderiv(id, x, &mut r);
        r
    }
}

//fp get_programiv
/// Get an integer value from a particular program
pub fn get_programiv(id: gl::types::GLuint, x: gl::types::GLuint) -> gl::types::GLint {
    unsafe {
        let mut r = 0;
        gl::GetProgramiv(id, x, &mut r);
        r
    }
}

//fp get_shader_error
/// Assummes an error exists; get its length (using 'f') and then get
/// the error (using 'e') as a String
pub fn get_shader_error<
    F: FnOnce(gl::types::GLuint) -> i32,
    E: FnOnce(gl::types::GLuint, i32, *mut gl::types::GLchar),
>(
    id: gl::types::GLuint,
    f: F,
    e: E,
) -> String {
    let len = f(id);
    let error = create_whitespace_cstring_with_len(len as usize);
    e(id, len, error.as_ptr() as *mut gl::types::GLchar);
    format!("{}", error.to_string_lossy())
}

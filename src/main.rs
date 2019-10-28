extern crate glfw;
extern crate gl;
use gl::types::*; 
use std::ffi::{CString, CStr};


use glfw::{Action, Context, Key};


fn bind_gl_funcs(window: &mut glfw::Window) {
    gl::Clear::load_with(|s| window.get_proc_address(s) as *const _);
    gl::ClearColor::load_with(|s| window.get_proc_address(s) as *const _);
    gl::GenBuffers::load_with(|s| window.get_proc_address(s) as *const _);
    gl::BindBuffer::load_with(|s| window.get_proc_address(s) as *const _);
    gl::BufferData::load_with(|s| window.get_proc_address(s) as *const _);
    gl::EnableVertexAttribArray::load_with(|s| window.get_proc_address(s) as *const _);
    gl::VertexAttribPointer::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DrawArrays::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DisableVertexAttribArray::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteBuffers::load_with(|s| window.get_proc_address(s) as *const _);

    gl::CreateShader::load_with(|s| window.get_proc_address(s) as *const _);
    gl::CreateProgram::load_with(|s| window.get_proc_address(s) as *const _);
    gl::ShaderSource::load_with(|s| window.get_proc_address(s) as *const _);
    gl::CompileShader::load_with(|s| window.get_proc_address(s) as *const _);
    gl::GetShaderiv::load_with(|s| window.get_proc_address(s) as *const _);
    gl::GetShaderInfoLog::load_with(|s| window.get_proc_address(s) as *const _);
    gl::AttachShader::load_with(|s| window.get_proc_address(s) as *const _);
    gl::LinkProgram::load_with(|s| window.get_proc_address(s) as *const _);
    gl::GetUniformLocation::load_with(|s| window.get_proc_address(s) as *const _);
    gl::UseProgram::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteShader::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteProgram::load_with(|s| window.get_proc_address(s) as *const _);



    // https://community.khronos.org/t/opengl-3-texturing/61558/4

}

fn create_shader(source: &CStr, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    unsafe { 
        let shader =  gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader); 
        let mut compilation_success: gl::types::GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compilation_success); 
        if compilation_success == gl::FALSE as gl::types::GLint {
            let mut log_size: gl::types::GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_size); 
            let mut log = Vec::<gl::types::GLchar>::with_capacity(log_size as usize);

            let mut actual_log_size: gl::types::GLint = 0;
            gl::GetShaderInfoLog(shader, log_size, &mut actual_log_size, log.as_mut_ptr()); 
            log.set_len(actual_log_size as usize);
            let u8_log = &*(log.as_slice() as *const [i8] as *const [u8]);
            let error = std::str::from_utf8(u8_log).unwrap();
            gl::DeleteShader(shader);
            return Err(String::from(error));
        }

        Ok(shader)
    }
}


struct QuadDrawer {
    vbo: gl::types::GLuint,
    vertices: [f32;20],

    vshader: gl::types::GLuint,
    fshader: gl::types::GLuint,
    program: gl::types::GLuint,
}


impl QuadDrawer {
    fn new() -> Result<Self,String> {
        let vertices = [
            -1.0f32, -1.0f32, 0.0f32, 1.0f32,
            -1.0f32,  1.0f32, 0.0f32, 1.0f32,
            0.9f32,  1.0f32, 0.0f32, 1.0f32,
            1.0f32, -1.0f32, 0.0f32, 1.0f32,
            -1.0f32, -1.0f32, 0.0f32, 1.0f32,
        ];

        let vshader = create_shader(&CString::new(include_str!("quad.vert")).unwrap(), gl::VERTEX_SHADER)?;
        let fshader = create_shader(&CString::new(include_str!("quad.frag")).unwrap(), gl::FRAGMENT_SHADER)?;

        let program = unsafe { gl::CreateProgram() };

        unsafe{
            gl::AttachShader(program, vshader);
            gl::AttachShader(program, fshader);
            gl::LinkProgram(program);
            gl::UseProgram(program);
        }

        let mut quad_drawer = QuadDrawer { 
            vbo: 0,
            vertices,
            vshader,
            fshader,
            program
        };
        let vbo_ptr = &mut quad_drawer.vbo as *mut gl::types::GLuint;

        unsafe {
            gl::GenBuffers(1, vbo_ptr);
            gl::BindBuffer(gl::ARRAY_BUFFER, quad_drawer.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>()*quad_drawer.vertices.len()) as isize, quad_drawer.vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        }
        Ok(quad_drawer)
    }

    fn draw(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 5);
            gl::DisableVertexAttribArray(0);
        }
    }
}

impl Drop for QuadDrawer {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteShader(self.vshader);
            gl::DeleteShader(self.fshader);
            gl::DeleteProgram(self.program);
        }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    println!("{:?}", glfw::get_version());
    println!("GLFW version: {:?}", glfw::get_version_string());

    let (mut window, events) = glfw.create_window(640, 480, "raytracer-rs", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    bind_gl_funcs(&mut window);

    let quad_drawer = QuadDrawer::new().unwrap();

    unsafe {
        gl::ClearColor(0.3,0.3,0.3,1.0);
    }

    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        quad_drawer.draw();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
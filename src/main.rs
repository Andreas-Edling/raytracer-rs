extern crate glfw;
extern crate gl;
use gl::types::*; 


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

}

struct Quad {
    vbo: gl::types::GLuint,
    vertices: [f32;20],
}

impl Quad {
    fn new() -> Self {
        let vertices = [
            -1.0f32, -1.0f32, 0.0f32, 1.0f32,
            -1.0f32,  1.0f32, 0.0f32, 1.0f32,
            0.9f32,  1.0f32, 0.0f32, 1.0f32,
            1.0f32, -1.0f32, 0.0f32, 1.0f32,
            -1.0f32, -1.0f32, 0.0f32, 1.0f32,
        ];
        let mut quad = Quad { 
            vbo: 0,
            vertices: vertices 
        };
        let vbo_ptr = &mut quad.vbo as *mut gl::types::GLuint;

        unsafe {
            gl::GenBuffers(1, vbo_ptr);
            gl::BindBuffer(gl::ARRAY_BUFFER, quad.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>()*quad.vertices.len()) as isize, quad.vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        }
        quad
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

impl Drop for Quad {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.vbo);
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

    let quad = Quad::new();

    unsafe {
        gl::ClearColor(0.3,0.3,0.3,1.0);
    }

    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        quad.draw();

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
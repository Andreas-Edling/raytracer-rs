extern crate gl;
extern crate glfw;
use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::{CStr, CString};

fn bind_gl_funcs(window: &mut glfw::Window) {
    gl::Clear::load_with(|s| window.get_proc_address(s) as *const _);
    gl::ClearColor::load_with(|s| window.get_proc_address(s) as *const _);
    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

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
    gl::Uniform1i::load_with(|s| window.get_proc_address(s) as *const _);
    gl::UseProgram::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteShader::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteProgram::load_with(|s| window.get_proc_address(s) as *const _);
    gl::GetAttribLocation::load_with(|s| window.get_proc_address(s) as *const _);

    gl::GenTextures::load_with(|s| window.get_proc_address(s) as *const _);
    gl::BindTexture::load_with(|s| window.get_proc_address(s) as *const _);
    gl::TexParameteri::load_with(|s| window.get_proc_address(s) as *const _);
    gl::TexImage2D::load_with(|s| window.get_proc_address(s) as *const _);
    gl::ActiveTexture::load_with(|s| window.get_proc_address(s) as *const _);
    gl::BindTexture::load_with(|s| window.get_proc_address(s) as *const _);
    gl::DeleteTextures::load_with(|s| window.get_proc_address(s) as *const _);
    // https://community.khronos.org/t/opengl-3-texturing/61558/4
}

fn create_shader(source: &CStr, shader_type: GLenum) -> Result<GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        let mut compilation_success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compilation_success);
        if compilation_success == gl::FALSE as GLint {
            let mut log_size: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_size);
            let mut log = Vec::<GLchar>::with_capacity(log_size as usize);

            let mut actual_log_size: GLint = 0;
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

struct Canvas {
    vbo: GLuint,
    vertices: [f32; 20],
    texture_data: Vec<u32>,
    texture_id: GLuint,

    vshader: GLuint,
    fshader: GLuint,
    program: GLuint,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Result<Self, String> {
        let vertices = [
            //x,y,u,v
            -1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32,
            1.0f32, 1.0f32, 1.0f32, -1.0f32, 1.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32,
        ];

        // shaders
        let vshader = create_shader(
            &CString::new(include_str!("quad.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )?;
        let fshader = create_shader(
            &CString::new(include_str!("quad.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )?;
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vshader);
            gl::AttachShader(program, fshader);
            gl::LinkProgram(program);
            gl::UseProgram(program);
        }

        let mut texture_data = vec![0u32; width * height];
        for i in 0..texture_data.len() {
            if i % 4 == 0 {
                texture_data[i] = 0xFFFF_FFFF;
            } else {
                texture_data[i] = 255;
            }
        }

        let mut canvas = Canvas {
            vbo: 0,
            vertices,
            texture_data,
            texture_id: 0,
            vshader,
            fshader,
            program,
        };

        // texture creation
        unsafe {
            gl::GenTextures(1, &mut canvas.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, canvas.texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                canvas.texture_data.as_ptr() as *const std::ffi::c_void,
            );

            let texloc = gl::GetUniformLocation(
                program,
                CString::new("tex_sampler").unwrap().as_ptr() as *const i8,
            );
            gl::Uniform1i(texloc, 0); // 0 for GL_TEXTURE0
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, canvas.texture_id);
        }

        // vbo creation
        let vbo_ptr = &mut canvas.vbo as *mut GLuint;
        unsafe {
            gl::GenBuffers(1, vbo_ptr);
            gl::BindBuffer(gl::ARRAY_BUFFER, canvas.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * canvas.vertices.len()) as isize,
                canvas.vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
            let position_location = gl::GetAttribLocation(
                canvas.program,
                CString::new("Position").unwrap().as_ptr() as *const i8,
            ) as GLuint;
            gl::VertexAttribPointer(
                position_location,
                4,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(position_location);
        }
        Ok(canvas)
    }

    fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 5);
        }
    }

    fn set_data(data: &[u32], width: i32, height: i32) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
        }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteShader(self.vshader);
            gl::DeleteShader(self.fshader);
            gl::DeleteProgram(self.program);
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

struct SoftCanvas {
    canvas: Canvas,
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl SoftCanvas {
    fn new(width: usize, height: usize) -> Result<Self, String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        println!("gl version: {:?}", glfw::get_version());

        let (mut window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                "raytracer-rs",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.make_current();

        bind_gl_funcs(&mut window);

        let canvas = Canvas::new(width as usize, height as usize).unwrap();

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        }

        Ok(SoftCanvas {
            canvas,
            glfw,
            window,
            events,
        })
    }

    fn is_running(&self) -> bool {
        !self.window.should_close()
    }

    fn stop_running(&mut self) {
        self.window.set_should_close(true);
    }

    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn draw(&mut self) {
        self.canvas.draw();
        self.window.swap_buffers();
    }

    fn handle_events<EventHandler: FnMut(glfw::WindowEvent, &mut Self)>(
        &mut self,
        mut event_handler: EventHandler,
    ) {
        self.glfw.poll_events();
        let events: Vec<(f64, glfw::WindowEvent)> = glfw::flush_messages(&self.events).collect();
        for (_, event) in events {
            match event {
                glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                },
                _ => event_handler(event, self),
            }
        }
    }

    fn set_data(&mut self, data: &[u32]) {
        let (w, h) = self.window.get_size();
        Canvas::set_data(data, w, h);
    }
}

fn main() {
    let mut soft_canvas = SoftCanvas::new(640, 480).unwrap();

    let data = vec![0xFF_00_00_FFu32; 640 * 480];
    Canvas::set_data(&data, 640, 480);

    // std::thread::spawn(move || {
    //     loop {
    //         Canvas::set_data(&data, 640, 480);
    //         std::thread::sleep(std::time::Duration::from_millis(50));
    //     }
    // });

    while soft_canvas.is_running() {
        soft_canvas.clear();
        soft_canvas.draw();

        soft_canvas.handle_events(|event, soft_canvas_context| match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                soft_canvas_context.stop_running();
            }
            _ => (),
        });
    }
}

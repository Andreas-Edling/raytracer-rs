mod raytracer;
mod vecmath;

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let mut soft_canvas = softcanvas::SoftCanvas::new(640, 480).expect("cant create canvas");
    let (frame_sender, frame_receiver) = std::sync::mpsc::channel();
    let (event_sender, event_receiver) = std::sync::mpsc::channel();
 
    std::thread::spawn(move || {
        let scene = raytracer::Scene::test_box();
        let mut raytracer = raytracer::RayTracer::new(WIDTH, HEIGHT, scene);
        let mut last_time = std::time::Instant::now();

        loop {
            let frame = raytracer.trace_frame();
            frame_sender.send(frame).expect("cant send data");

            for event in event_receiver.try_iter() {
                match event {
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::Left, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.move_rel(0.1,0.0,0.0);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::Right, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.move_rel(-0.1,0.0,0.0);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::Up, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.move_rel(0.0, 0.1, 0.0);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::Down, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.move_rel(0.0, -0.1, 0.0);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::A, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.add_y_angle(0.01);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::D, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.add_y_angle(-0.01);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::W, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.add_x_angle(0.01);
                    },
                    softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::S, _, softcanvas::glfw::Action::Press, _) => {
                        raytracer.camera.add_x_angle(-0.01);
                    },
                    _ => (),
                }
            }

            let now = std::time::Instant::now();
            let frame_duration: std::time::Duration = now - last_time;
            last_time = now;
            println!("frame duration: {:?}", frame_duration);
        }
    });

    while soft_canvas.is_running() {
        if let Ok(frame) = frame_receiver.try_recv() {
            soft_canvas.set_data(&frame);
        }

        soft_canvas.clear();
        soft_canvas.draw();

        #[rustfmt::skip]
        soft_canvas.handle_events(|event, soft_canvas_context| match event {
            softcanvas::glfw::WindowEvent::Key(softcanvas::glfw::Key::Escape, _, softcanvas::glfw::Action::Press, _) => {
                soft_canvas_context.stop_running();
            }
            _ => { event_sender.send(event); },
        });
    }
}

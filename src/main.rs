mod raytracer;

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let mut soft_canvas = softcanvas::SoftCanvas::new(640, 480).expect("cant create canvas");
    let (frame_sender, frame_receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let scene = raytracer::Scene::test_scene();
        let raytracer = raytracer::RayTracer::new(WIDTH, HEIGHT, scene);

        loop {
            let frame = raytracer.trace_frame();
            frame_sender.send(frame).expect("cant send data");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    while soft_canvas.is_running() {
        if let Ok(frame) = frame_receiver.try_recv() {
            soft_canvas.set_data(&frame);
        }

        soft_canvas.clear();
        soft_canvas.draw();

        soft_canvas.handle_events(|event, soft_canvas_context| match event {
            softcanvas::glfw::WindowEvent::Key(
                softcanvas::glfw::Key::Escape,
                _,
                softcanvas::glfw::Action::Press,
                _,
            ) => {
                soft_canvas_context.stop_running();
            }
            _ => (),
        });
    }
}

mod raytracer;
mod scene;
mod tonemap;
mod vecmath;

use std::{
    sync::mpsc::channel,
    sync::{Arc, RwLock},
};

use minifb::{Key, Window, WindowOptions};
use clap::{Arg, App};

#[allow(unused_imports)]
use scene::loaders::{boxloader::BoxLoader, colladaloader::ColladaLoader, SceneLoader};

use timing::BenchMark;

#[derive(Debug, Clone, Copy)]
enum Event {
    KeyDown(Key),
}

fn generate_events(window: &Window) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(keys) = window.get_keys() {
        for key in keys {
            events.push(Event::KeyDown(key));
        }
    }

    events
}

fn main() -> Result<(), String> {
    let matches = App::new("raytracer-rs")
        .version("0.1.0")
        .author("Andreas Edling")
        .arg(Arg::with_name("collada_file")
            .short("f")
            .long("file")
            .value_name("COLLADA_FILENAME")
            .help("what collada (.dae) file to load for rendering")
        )
        .arg(Arg::with_name("max_triangles")
            .short("m")
            .long("max_triangles")
            .value_name("MAX_TRIS")
            .help(&format!("sets maximum number of triangles per leaf in octtree. defaults to {} if omitted",raytracer::accel_intersect::oct_tree_intersector::DEFAULT_TRIANGLES_PER_LEAF))
        )
        .arg(Arg::with_name("frame_iterations")
            .short("f")
            .long("frame_iterations")
            .value_name("FRAME_ITERATIONS")
            .help(&format!("sets a bound on how many frame iterations should be calculated."))
        )
        .get_matches();

    let max_triangles = match matches.value_of("max_triangles") {
        Some(max_triangles) => max_triangles.parse::<usize>().unwrap_or(raytracer::accel_intersect::oct_tree_intersector::DEFAULT_TRIANGLES_PER_LEAF),
        None => raytracer::accel_intersect::oct_tree_intersector::DEFAULT_TRIANGLES_PER_LEAF,
    };
    println!("max triangles per leaf: {}",max_triangles);

    let frame_iterations = match matches.value_of("frame_iterations") {
        Some(frame_iterations) => frame_iterations.parse::<usize>().ok(),
        None => None,
    };
    if let Some(frame_iterations) = frame_iterations {
        println!("will quit after {} frame iterations", frame_iterations);
    }

    let filename = match matches.value_of("collada_file") {
        Some(collada_file) => collada_file,
        None => "./data/ico2.dae",
    };

    // setup
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    let mut window = Window::new("raytracer-rs", WIDTH, HEIGHT, WindowOptions::default())
        .map_err(|e| e.to_string())?;
    let frame = Arc::new(RwLock::new(vec![0u32; WIDTH * HEIGHT]));
    let (copy_frame_sender, copy_frame_reciever) = channel();
    let (copied_frame_sender, copied_frame_reciever) = channel();
    let (events_sender, events_receiver): (
        std::sync::mpsc::Sender<Vec<Event>>,
        std::sync::mpsc::Receiver<Vec<Event>>,
    ) = std::sync::mpsc::channel();
    let (shutdown_sender, shutdown_receiver) = std::sync::mpsc::channel();
    let mut fps = Fps::new();
    let mut current_iteration = 0;
    let scene = ColladaLoader::from_file(filename).map_err(|e| e.to_string())?;
    let octtree = raytracer::accel_intersect::OctTreeIntersector::with_triangles_per_leaf(&scene, max_triangles);
    let mut raytracer = raytracer::RayTracer::new_with_intersector(WIDTH, HEIGHT, scene.cameras[0].clone(), octtree);

    // raytracer loop
    let raytracer_thread = std::thread::spawn({
        let frame = Arc::clone(&frame);
        move || {
            let mut timer = BenchMark::new();

            let mut running = true;
            while running {
                timer.start("trace_frame_additive");
                raytracer.trace_frame_additive(&scene, &mut timer);
                timer.stop("trace_frame_additive");

                timer.start("get_film");
                let generated_frame = raytracer.film.get_film();
                timer.stop("get_film");

                timer.start("frame_to_u32");
                let ldr_frame = generated_frame
                    .iter()
                    .map(|pix| tonemap::simple_map(pix))
                    .map(|pix| scene::color::RGBA::from_rgb(pix, 1.0).to_u32())
                    .collect();
                timer.stop("frame_to_u32");

                // lock & copy frame
                timer.start("lock_&_copy");
                {
                    let mut frame_w = frame.write().unwrap();
                    *frame_w = ldr_frame;
                }
                timer.stop("lock_&_copy");

                // notify main thread
                copy_frame_sender
                    .send(())
                    .expect("channel copy_frame_sender failed on send");

                //wait for main thread
                copied_frame_reciever
                    .recv()
                    .expect("channel copied_frame_reciever failed on recv");

                for events in events_receiver.try_iter() {
                    for event in events {
                        match event {
                            Event::KeyDown(key) => match key {
                                Key::Left => {
                                    raytracer.camera.move_rel(0.1, 0.0, 0.0);
                                    raytracer.film.clear();
                                }
                                Key::Right => {
                                    raytracer.camera.move_rel(-0.1, 0.0, 0.0);
                                    raytracer.film.clear();
                                }
                                Key::Up => {
                                    raytracer.camera.move_rel(0.0, 0.1, 0.0);
                                    raytracer.film.clear();
                                }
                                Key::Down => {
                                    raytracer.camera.move_rel(0.0, -0.1, 0.0);
                                    raytracer.film.clear();
                                }
                                Key::Comma => {
                                    raytracer.camera.move_rel(0.0, 0.0, 0.1);
                                    raytracer.film.clear();
                                }
                                Key::Period => {
                                    raytracer.camera.move_rel(0.0, 0.0, -0.1);
                                    raytracer.film.clear();
                                }
                                Key::A => {
                                    raytracer.camera.add_y_angle(0.01);
                                    raytracer.film.clear();
                                }
                                Key::D => {
                                    raytracer.camera.add_y_angle(-0.01);
                                    raytracer.film.clear();
                                }
                                Key::W => {
                                    raytracer.camera.add_x_angle(0.01);
                                    raytracer.film.clear();
                                }
                                Key::S => {
                                    raytracer.camera.add_x_angle(-0.01);
                                    raytracer.film.clear();
                                }
                                _ => (),
                            },
                        }
                    }
                }

                println!("fps: {:?} ", fps.fps());

                if shutdown_receiver.try_recv().is_ok() {
                    running = false;
                }
            }
            println!("{}", timer);
            println!("mean fps {}\n\n", fps.mean());
        }
    });

    // main / gui loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // update window with frame if available
        if copy_frame_reciever.try_recv().is_ok() {
            // lock & update
            {
                let frame_r = frame.read().unwrap();
                window
                    .update_with_buffer_size(&frame_r, WIDTH, HEIGHT)
                    .unwrap();
            }
            copied_frame_sender
                .send(())
                .expect("channel copied_frame_sender failed on send");


            if let Some(frame_iterations) = frame_iterations {
                current_iteration += 1;
                if current_iteration >= frame_iterations {
                    break;
                }
            }
        }

        window.update();

        //send events
        let events = generate_events(&window);
        events_sender
            .send(events)
            .expect("channel events_sender failed on send");

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    // shutdown
    copied_frame_sender
        .send(())
        .expect("channel copied_frame_sender failed on send"); //we need to send this to unblock raytracer thread
    shutdown_sender
        .send(())
        .expect("unable to send shutdown signal");
    raytracer_thread
        .join()
        .expect("couldnt join raytracer thread");
    Ok(())
}

struct Fps {
    last_iteration: std::time::Instant,
    fps_sum: f32,
    num_measurements: u32,
}

impl Fps {
    fn new() -> Self {
        let last_iteration = std::time::Instant::now();
        Fps { last_iteration, fps_sum: 0.0, num_measurements: 0 }
    }

    fn fps(&mut self) -> f32 {
        let now = std::time::Instant::now();
        let frame_duration: std::time::Duration = now - self.last_iteration;
        self.last_iteration = now;
        let fps = 1.0 / frame_duration.as_secs_f32();
        self.fps_sum += fps;
        self.num_measurements += 1;
        fps
    }

    fn mean(&self) -> f32 {
        self.fps_sum / self.num_measurements as f32
    }
}

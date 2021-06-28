
use raytracer_lib::{RayTracer, stats::Stats};

use std::{
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, RwLock},
    time::Duration,
};

use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};

const DEFAULT_WIDTH: usize = 1024;
const DEFAULT_HEIGHT: usize = 768;
const DEFAULT_COLLADA_FILE: &str = "./data/thai2.dae";
//const DEFAULT_COLLADA_FILE: &str = "./data/4boxes.dae";


struct CmdArgs {
    max_triangles: usize,
    frame_iterations: Option<usize>,
    collada_filename: String,
    width: usize,
    height: usize,
}

impl CmdArgs {
    pub fn get_cmd_args() -> CmdArgs {
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
            .help(&format!("sets maximum number of triangles per leaf in octtree. defaults to {} if omitted",raytracer_lib::DEFAULT_TRIANGLES_PER_LEAF))
        )
        .arg(Arg::with_name("frame_iterations")
            .short("i")
            .long("frame_iterations")
            .value_name("FRAME_ITERATIONS")
            .help("sets a bound on how many frame iterations should be calculated.")
        )
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("WIDTH")
            .help("sets width of output")
        )
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("HEIGHT")
            .help("sets height of output")
        )
        .get_matches();

        let max_triangles = match matches.value_of("max_triangles") {
            Some(max_triangles) => max_triangles.parse::<usize>().unwrap_or(
                raytracer_lib::DEFAULT_TRIANGLES_PER_LEAF,
            ),
            None => raytracer_lib::DEFAULT_TRIANGLES_PER_LEAF,
        };
        println!("max triangles per leaf: {}", max_triangles);

        let frame_iterations = match matches.value_of("frame_iterations") {
            Some(frame_iterations) => frame_iterations.parse::<usize>().ok(),
            None => None,
        };
        if let Some(frame_iterations) = frame_iterations {
            println!("will quit after {} frame iterations", frame_iterations);
        }

        let width = match matches.value_of("width") {
            Some(width) => width.parse::<usize>().unwrap_or(DEFAULT_WIDTH),
            None => DEFAULT_WIDTH,
        };

        let height = match matches.value_of("height") {
            Some(height) => height.parse::<usize>().unwrap_or(DEFAULT_HEIGHT),
            None => DEFAULT_HEIGHT,
        };

        let collada_filename = match matches.value_of("collada_file") {
            Some(collada_file) => collada_file,
            None => DEFAULT_COLLADA_FILE,
        }
        .to_string();

        CmdArgs {
            max_triangles,
            frame_iterations,
            collada_filename,
            width,
            height,
        }
    }
}

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

fn handle_events(
    raytracer: &mut RayTracer,
    events_receiver: &Receiver<Vec<Event>>,
) {
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
}


fn main() -> Result<(), String> {
    let cmd_args = CmdArgs::get_cmd_args();

    // setup
    let (width, height) = (cmd_args.width, cmd_args.height);
    let mut window = Window::new("raytracer-rs", width, height, WindowOptions::default())
        .map_err(|e| e.to_string())?;
    let frame = Arc::new(RwLock::new(vec![0u32; width*height]));
    let (events_sender, events_receiver): (Sender<Vec<Event>>, Receiver<Vec<Event>>) = channel();
    let mut stats = Stats::new();
    let mut current_iteration = 0;
    let mut raytracer = raytracer_lib::create_raytracer_from_file(
        cmd_args.collada_filename, 
        cmd_args.max_triangles, 
        width, 
        height)?;

    let (frame_ready_signaler, frame_ready_listener) = waithandle::new();
    let (copied_frame_signaler, copied_frame_listener) = waithandle::new();
    let (shutdown_signaler, shutdown_listener) = waithandle::new();

    // raytracer loop
    let raytracer_thread = std::thread::spawn({
        let frame = Arc::clone(&frame);
        move || {
            while !shutdown_listener.check().expect("shutdown_listener failed") {

                // render
                let num_primary_rays = raytracer.trace_frame_additive();
                let ldr_frame = raytracer.get_tonemapped_pixels();

                // lock & copy frame, notify, wait
                {
                    let mut frame_w = frame.write().unwrap();
                    *frame_w = ldr_frame;
                }
                frame_ready_signaler.signal().expect("frame_ready_signaler failed");
                copied_frame_listener.wait(Duration::from_millis(10000)).expect("copied_frame_listener failed");

                handle_events(&mut raytracer, &events_receiver);

                println!("{}", stats.stats(num_primary_rays));
            }

            println!("{}\n\n", stats.mean_stats());
        }
    });

    // main / gui loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // update screen with frame if it's available, 
        // or timeout and update window, mainting 60Hz-ish framerate, 
        // to keep the window responsive
        if frame_ready_listener.wait(Duration::from_millis(16)).expect("frame_ready_listener failed") {
            // lock & update
            {
                let frame_r = frame.read().unwrap();
                window
                    .update_with_buffer_size(&frame_r, width, height)
                    .unwrap();
            }

            //notify raytracer
            copied_frame_signaler.signal().expect("copied_frame_signaler failed");

            if let Some(frame_iterations) = cmd_args.frame_iterations {
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
    }

    // shutdown
    shutdown_signaler.signal().expect("unable to send shutdown signal");
    raytracer_thread
        .join()
        .expect("couldn't join raytracer thread");
    Ok(())
}


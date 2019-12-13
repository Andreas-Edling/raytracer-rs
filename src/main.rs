
mod raytracer;
mod scene;
mod vecmath;

use std::fs::File;
use std::{
    io::prelude::*,
    sync::mpsc::channel,
    sync::{Arc, RwLock},
};

use minifb::{Key, WindowOptions, Window};

#[allow(unused_imports)]
use scene::loaders::{
    SceneLoader,
    boxloader::BoxLoader,
    colladaloader::ColladaLoader,
};

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


fn main() -> Result<(), String>{
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let mut window = Window::new("raytracer-rs", WIDTH, HEIGHT, WindowOptions::default()).map_err(|e| e.to_string())?;

    let frame = Arc::new(RwLock::new(vec![0u32; WIDTH * HEIGHT]));
    let (copy_frame_sender, copy_frame_reciever) = channel();
    let (copied_frame_sender, copied_frame_reciever) = channel();

    let (events_sender, events_receiver): (std::sync::mpsc::Sender<Vec<Event>>, std::sync::mpsc::Receiver<Vec<Event>>) = std::sync::mpsc::channel();

    let contents = {
        let mut file = File::open("./data/4boxes.dae").map_err(|e| e.to_string())?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
        contents
    };
    let scene = ColladaLoader::from_str(&contents).map_err(|e| format!("{:?}",e))?;
    let mut raytracer = raytracer::RayTracer::new(WIDTH, HEIGHT, scene);

    #[rustfmt::skip]
    std::thread::spawn({
        let frame = Arc::clone(&frame);

        move || {
            let mut last_time = std::time::Instant::now();

            loop {
                let generated_frame = raytracer.trace_frame(); 

                // lock & copy data
                {
                    let mut frame_w = frame.write().unwrap();
                    *frame_w = generated_frame; // TODO - get rid of this copy.
                }

                // notify main thread
                copy_frame_sender.send(()).expect("channel copy_frame_sender failed on send");

                //wait for main thread
                copied_frame_reciever.recv().expect("channel copied_frame_reciever failed on recv");

                
                for events in events_receiver.try_iter() {
                    for event in events {
                        match event {
                            Event::KeyDown(key) => match key { 
                                Key::Left => raytracer.camera.move_rel(0.1,0.0,0.0),
                                Key::Right => raytracer.camera.move_rel(-0.1,0.0,0.0),
                                Key::Up => raytracer.camera.move_rel(0.0, 0.1, 0.0),
                                Key::Down => raytracer.camera.move_rel(0.0, -0.1, 0.0),
                                Key::Comma => raytracer.camera.move_rel(0.0, 0.0, 0.1),
                                Key::Period => raytracer.camera.move_rel(0.0, 0.0, -0.1),
                                Key::A => raytracer.camera.add_y_angle(0.01),
                                Key::D => raytracer.camera.add_y_angle(-0.01),
                                Key::W => raytracer.camera.add_x_angle(0.01),
                                Key::S => raytracer.camera.add_x_angle(-0.01),
                                _ => (),
                            },
                        }
                    }
                }

                let now = std::time::Instant::now();
                let frame_duration: std::time::Duration = now - last_time;
                last_time = now;
                println!("fps: {:?}  frame duration: {:?}", 1.0/frame_duration.as_secs_f32(), frame_duration);
            }
        }
    });
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // update windowwith frame if available
        if copy_frame_reciever.try_recv().is_ok() {
            
            // lock & update
            { 
                let frame_r = frame.read().unwrap();
                window.update_with_buffer_size(&frame_r, WIDTH, HEIGHT).unwrap();
            }
            copied_frame_sender.send(()).expect("channel copied_frame_sender failed on send");
        }

        window.update();

        //send events
        let events = generate_events(&window);
        events_sender.send(events).expect("channel events_sender failed on send");

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

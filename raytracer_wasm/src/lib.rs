use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

mod inline_data;

use raytracer_lib::RayTracer;

#[wasm_bindgen]
pub struct RaytracerProxy {
    raytracer: RayTracer,
}

#[wasm_bindgen]
pub fn create_raytracer() -> RaytracerProxy {
    RaytracerProxy{
        raytracer: raytracer_lib::create_raytracer(
            inline_data::BOXES_DOC,
            70,
            1024,
            768
        ).unwrap()
    }
}

#[wasm_bindgen]
pub fn draw_traced(raytracer_proxy: &mut RaytracerProxy) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    raytracer_proxy.raytracer.trace_frame_additive();
    let pixels = raytracer_proxy.raytracer.get_tonemapped_pixels();

    let mut pixels_u8 = Vec::with_capacity(pixels.len()*4);
    for pix in pixels {
        pixels_u8.push((pix & 0xFF0000).wrapping_shr(16) as u8);
        pixels_u8.push((pix & 0xFF00).wrapping_shr(8) as u8);
        pixels_u8.push((pix & 0xFF) as u8);
        pixels_u8.push((pix & 0xFF000000).wrapping_shr(24) as u8);
    }
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels_u8[..]), 1024, 768).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();
}

// #[wasm_bindgen]
// pub fn trace_frame_additive(raytracer_proxy: &mut RaytracerProxy) -> u32 {
//     raytracer_proxy.raytracer.trace_frame_additive()
// }

// #[wasm_bindgen]
// pub fn get_tonemapped_pixels(raytracer_proxy: &RaytracerProxy) -> Vec<u32> {
//     raytracer_proxy.raytracer.get_tonemapped_pixels()
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use crate::scene::color::RGB;

#[derive(Clone)]
pub struct PixelAndSampleCount {
    pub pixel: RGB,
    pub num_samples: u32,
}
impl Default for PixelAndSampleCount {
    fn default() -> Self {
        PixelAndSampleCount {
            pixel: RGB::black(),
            num_samples: 0,
        }
    }
}

impl PixelAndSampleCount {
    pub fn add_sample(&mut self, rgb: RGB) {
        self.pixel += rgb;
        self.num_samples += 1;
    }
}

pub struct Film {
    pub pixels_and_sample_counts: Vec<PixelAndSampleCount>,
}

impl Film {
    pub fn new(size: usize) -> Self {
        let pixels_and_sample_counts = vec![Default::default(); size];
        Film {
            pixels_and_sample_counts,
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixels_and_sample_counts.iter_mut() {
            *p = Default::default();
        }
    }

    pub fn get_film(&self) -> Vec<RGB> {
        self.pixels_and_sample_counts
            .iter()
            .map(|pixel_sum_and_num_samples| {
                pixel_sum_and_num_samples.pixel
                    * (1.0 / pixel_sum_and_num_samples.num_samples as f32)
            })
            .collect()
    }
}

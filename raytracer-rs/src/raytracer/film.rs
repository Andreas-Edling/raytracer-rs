use crate::scene::color::RGB;

#[derive(Clone)]
pub struct PixelData {
    pub pixel_sum: RGB,
    pub pixel_sum_squared: RGB,
    pub num_samples: u32,
}
impl Default for PixelData {
    fn default() -> Self {
        PixelData {
            pixel_sum: RGB::black(),
            pixel_sum_squared: RGB::black(),
            num_samples: 0,
        }
    }
}

impl PixelData {
    pub fn add_sample(&mut self, rgb: RGB) {
        self.pixel_sum += rgb;
        self.pixel_sum_squared += RGB::new(rgb.r * rgb.r, rgb.g * rgb.g, rgb.b * rgb.b);
        self.num_samples += 1;
    }
}

pub struct Film {
    pub pixel_datas: Vec<PixelData>,
}

impl Film {
    pub fn new(size: usize) -> Self {
        let pixel_datas = vec![Default::default(); size];
        Film {
            pixel_datas,
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixel_datas.iter_mut() {
            *p = Default::default();
        }
    }

    pub fn get_pixels(&self) -> Vec<RGB> {
        self.pixel_datas
            .iter()
            .map(|pixel_data| {
                pixel_data.pixel_sum
                    * (1.0 / pixel_data.num_samples as f32)
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_estimated_variances(&self) -> Vec<RGB> {
        self.pixel_datas
            .iter()
            .map(|pixel_data| {
                let n_times_n_minus_1 = (pixel_data.num_samples * (pixel_data.num_samples-1)) as f32;
                let n_squared_times_n_minus_1 = pixel_data.num_samples as f32 * n_times_n_minus_1;
                let variance_r = pixel_data.pixel_sum_squared.r / n_times_n_minus_1
                    - pixel_data.pixel_sum.r * pixel_data.pixel_sum.r / n_squared_times_n_minus_1;
                let variance_g = pixel_data.pixel_sum_squared.g / n_times_n_minus_1
                    - pixel_data.pixel_sum.g * pixel_data.pixel_sum.g / n_squared_times_n_minus_1;
                let variance_b = pixel_data.pixel_sum_squared.b / n_times_n_minus_1
                    - pixel_data.pixel_sum.b * pixel_data.pixel_sum.b / n_squared_times_n_minus_1;
                RGB::new(variance_r*50.0, variance_g*50.0, variance_b*50.0)
            })
            .collect()

    }
}

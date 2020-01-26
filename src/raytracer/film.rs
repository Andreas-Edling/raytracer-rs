use crate::scene::color::RGB;

pub struct Film {
    pixels: Vec<RGB>,
    num_samples: Vec<u32>,
}

impl Film {
    pub fn new(size: usize) -> Self {
        let pixels = vec![RGB::black(); size];
        let num_samples = vec![0; size];
        Film {
            pixels,
            num_samples,
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixels.iter_mut() {
            *p = RGB::black();
        }
        for s in self.num_samples.iter_mut() {
            *s = 0;
        }

    }

    pub fn iter_mut<'a>(&'a mut self) -> FilmIterMut<'a> {
        FilmIterMut {
            pixel_iter: self.pixels.iter_mut(),
            num_samples_iter: self.num_samples.iter_mut(),
        }
    }

    pub fn get_film(&self) -> Vec<RGB> {
        self.pixels
            .iter()
            .zip(self.num_samples.iter())
            .map(|(rgb_sum, num_samples)| rgb_sum * (1.0 / *num_samples as f32))
            .collect()
    }
}

pub struct FilmIterMut<'a> {
    pixel_iter: std::slice::IterMut<'a, RGB>,
    num_samples_iter: std::slice::IterMut<'a, u32>,
}

impl<'a> Iterator for FilmIterMut<'a> {
    type Item = (&'a mut u32, &'a mut RGB);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.num_samples_iter.next(), self.pixel_iter.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(_), None) => None,
            (None, Some(_)) => None,
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration() {
        let mut film = Film::new(100);
        let mut iter = film.iter_mut();
        for (count, rgb) in film.iter_mut() {
            *count += 10;
        }

        for (count, rgb) in film.iter_mut() {
            assert_eq!(*count, 10);
            assert_eq!(*rgb, RGB::black());
        }
    }
}

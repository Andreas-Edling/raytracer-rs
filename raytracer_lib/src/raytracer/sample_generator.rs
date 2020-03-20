use crate::vecmath::{dot, Vec3};
use rand::Rng;

type SampleIdx = u16;
const SAMPLE_MAX: SampleIdx = std::u16::MAX;

const NUM_SAMPLES: usize = SAMPLE_MAX as usize + 1;

pub struct SampleGenerator {
    normalized_vecs: Vec<Vec3>,
    sample_idx: SampleIdx,
}

impl SampleGenerator {
    pub fn new() -> Self {
        let normalized_vecs: Vec<Vec3> = (0..NUM_SAMPLES)
            .map(|_| Self::generate_normalized_vec3())
            .collect();

        SampleGenerator {
            normalized_vecs,
            sample_idx: 0,
        }
    }

    pub fn normalized_vec_lookup(&mut self) -> Vec3 {
        self.sample_idx = (self.sample_idx + 1) % SAMPLE_MAX;
        self.normalized_vecs[self.sample_idx as usize]
    }

    pub fn normalized_vec_pseudo(&mut self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        self.sample_idx = rng.gen_range(0, NUM_SAMPLES - 1) as u16;
        self.normalized_vecs[self.sample_idx as usize]
    }

    fn generate_normalized_vec3() -> Vec3 {
        let mut rng = rand::thread_rng();

        // randomize in box, until inside unit sphere, then normalize
        let dir = loop {
            let dir = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );
            if dot(&dir, &dir) < 1.0 {
                break dir;
            }
        };

        dir.normalized()
    }
}

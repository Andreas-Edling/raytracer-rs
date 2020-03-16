

pub struct Stats {
    last_iteration: std::time::Instant,
    fps_sum: f32,
    primrays_per_sec_sum: f32,
    num_measurements: u32,
}

impl Stats {
    pub fn new() -> Self {
        let last_iteration = std::time::Instant::now();
        Self {
            last_iteration,
            fps_sum: 0.0,
            primrays_per_sec_sum: 0.0,
            num_measurements: 0,
        }
    }

    pub fn stats(&mut self, num_primary_rays: u32) -> String {
        let now = std::time::Instant::now();
        let frame_duration: std::time::Duration = now - self.last_iteration;
        self.last_iteration = now;
        let fps = 1.0 / frame_duration.as_secs_f32();
        self.fps_sum += fps;
        let primrays_per_sec = num_primary_rays as f32 / frame_duration.as_secs_f32();
        self.primrays_per_sec_sum += primrays_per_sec;
        self.num_measurements += 1;
        format!("fps: {}  primary rays/s: {}", fps, primrays_per_sec as u32)
    }

    pub fn mean_stats(&self) -> String {
        format!(
            "mean fps: {}  mean primary rays/s: {}",
            self.fps_sum / self.num_measurements as f32,
            self.primrays_per_sec_sum / self.num_measurements as f32
        )
    }
}

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Debug, Default, Clone)]
pub struct BenchMark {
    timings: HashMap<&'static str, Timing>,
}

impl BenchMark {
    pub fn new() -> Self {
        BenchMark {
            timings: HashMap::new(),
        }
    }

    pub fn start(&mut self, name: &'static str) {
        let now = Instant::now();
        self.timings
            .entry(name)
            .and_modify(|timing| timing.start = now)
            .or_insert_with(|| Timing::new(now));
    }

    pub fn stop(&mut self, name: &'static str) {
        let timing = self
            .timings
            .get_mut(name)
            .ok_or("")
            .expect("unexpected name in stop()");

        timing.total_duration += Instant::now() - timing.start;
        timing.samples += 1;
    }

    pub fn time_scope(&mut self, name: &'static str) -> Scope {
        self.start(&name);
        Scope {
            benchmark: self,
            name,
        }
    }

    pub fn collect_timing_results(&self) -> Vec<TimingResult> {
        let mut timing_results = self
            .timings
            .iter()
            .map(|(name, timing)| TimingResult {
                name,
                total_duration: timing.total_duration,
                mean_duration: timing.total_duration / timing.samples as u32,
                samples: timing.samples,
            })
            .collect::<Vec<TimingResult>>();
        timing_results.sort_unstable_by(|a, b| b.total_duration.cmp(&a.total_duration));
        timing_results
    }
}

#[derive(Debug, Clone)]
pub struct Timing {
    start: Instant,
    total_duration: Duration,
    samples: usize,
}
impl Timing {
    pub fn new(start: Instant) -> Self {
        Timing {
            start,
            total_duration: Duration::from_millis(0),
            samples: 0,
        }
    }
}

pub struct TimingResult<'a> {
    name: &'a str,
    total_duration: Duration,
    mean_duration: Duration,
    samples: usize,
}

#[derive(Debug)]
pub struct Scope<'a> {
    benchmark: &'a mut BenchMark,
    name: &'static str,
}
impl<'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        self.benchmark.stop(&self.name);
    }
}

impl std::fmt::Display for BenchMark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for timing_result in self.collect_timing_results() {
            writeln!(
                f,
                "{} total: {}ms, mean: {}ms, samples: {}",
                timing_result.name,
                timing_result.total_duration.as_micros() as f32 / 1000.0,
                timing_result.mean_duration.as_micros() as f32 / 1000.0,
                timing_result.samples
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_benchmark() {
        // run with 'cargo test -- --nocapture'

        let mut benchmark = BenchMark::new();

        benchmark.start("foo");
        std::thread::sleep(Duration::from_millis(200));
        benchmark.stop("foo");

        for _i in 0..10 {
            benchmark.start("bar");
            std::thread::sleep(Duration::from_millis(20));
            benchmark.stop("bar");

            // ..other stuff here

            benchmark.start("bar");
            std::thread::sleep(Duration::from_millis(20));
            benchmark.stop("bar");
        }

        {
            let _scope = benchmark.time_scope("scope");
            std::thread::sleep(Duration::from_millis(200));
        }

        for _i in 0..10 {
            let _scope_loop = benchmark.time_scope("scope_loop");
            std::thread::sleep(Duration::from_millis(20));
        }

        println!("{}", benchmark);
    }

    #[test]
    fn test_seq() {
        let mut benchmark = BenchMark::new();

        benchmark.start("one");
        benchmark.stop("one");

        benchmark.start("two");
        benchmark.stop("two");

        println!("{}", benchmark);
    }
}

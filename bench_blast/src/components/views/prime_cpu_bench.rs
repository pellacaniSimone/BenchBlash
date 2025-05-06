#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use web_sys::window;

pub fn cpu() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let start = Instant::now();
        let timeout = Duration::from_secs(15);
        compute_benchmark(|| start.elapsed() < timeout)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let window = window().expect("no global `window` exists");
        let performance = window.performance().expect("performance should be available");
        let start = performance.now();
        let timeout = 15_000.0; // 15 seconds in milliseconds
        compute_benchmark(|| performance.now() - start < timeout)
    }
}

fn compute_benchmark<F>(mut within_time: F) -> String
where
    F: FnMut() -> bool,
{
    let mut p: u32 = 2;
    let mut tested: u32 = 0;
    let mut fractional: f64 = 0.0;

    while within_time() {
        let mp: u128 = (1 << p) - 1;
        let sqrt: u128 = (mp as f64).sqrt() as u128;

        let mut i: u128 = 2;
        while i <= sqrt {
            if mp % i == 0 { break; }
            if !within_time() {
                fractional = i as f64 / sqrt as f64;
                break;
            }
            i += 1;
        }
        if within_time() { tested += 1; }
        p += 1;
    }

    let score = tested as f64 + fractional;
    score.to_string()
}

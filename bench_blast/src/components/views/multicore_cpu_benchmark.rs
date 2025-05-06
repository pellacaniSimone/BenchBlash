use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::{Duration, Instant};

// https://doc.rust-lang.org/nomicon/index.html
// https://doc.rust-lang.org/book/ch16-03-shared-state.html
const MAX_THREADS: usize = 4;
const TIME_LIMIT: Duration = Duration::from_secs(2);

// contatore globale per i numeri di Mersenne
static SAFE_COUNTER: AtomicU32 = AtomicU32::new(2);

fn next_exponent() -> u32 {
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn mersenne(p: u32) -> u128 {
    (1u128 << p) - 1
}

fn is_prime_mersenne(mp: u128, stop: Instant) -> f64 {
    let sqrt = (mp as f64).sqrt() as u128;

    for i in 2..=sqrt {
        if Instant::now() > stop {
            return i as f64 / sqrt as f64;
        }
        if mp % i == 0 {
            return 1.0;
        }
    }
    1.0
}

fn compute_benchmark() -> String {
    let scores = Arc::new(Mutex::new(vec![0.0; MAX_THREADS]));
    let deadline = Instant::now() + TIME_LIMIT;

    let mut handles = vec![];

    for thread_id in 0..MAX_THREADS {
        let scores = Arc::clone(&scores);
        let handle = thread::spawn(move || {
            let mut score = 0.0;
            while Instant::now() < deadline {
                let p = next_exponent();
                let mp = mersenne(p);
                score += is_prime_mersenne(mp, deadline);
            }
            scores.lock().unwrap()[thread_id] = score;
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let total: f64 = scores.lock().unwrap().iter().sum();
    let avg = total / MAX_THREADS as f64;

    avg.to_string()
}

pub fn cpu_mt() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        compute_benchmark()
    }

    #[cfg(target_arch = "wasm32")]
    {
        compute_benchmark() // used
    }
}
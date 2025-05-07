use std::sync::{Arc, Mutex};
use std::thread;

const TIME_LIMIT_SECS: u64 = 5;
const BASE: u128 = 2;
static SAFE_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(2);
const MAX_THREADS: usize = 2;

fn next_exponent() -> u32 {
    SAFE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn mersenne() -> u128 {
    BASE.pow(next_exponent()) - 1
}

#[cfg(not(target_arch = "wasm32"))]
fn remaining_timer() -> impl FnMut() -> bool {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(TIME_LIMIT_SECS);
    move || start.elapsed() < timeout
}

#[cfg(target_arch = "wasm32")]
fn remaining_timer() -> impl FnMut() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let performance = window.performance().expect("performance should be available");
    let start = performance.now();
    let timeout = (TIME_LIMIT_SECS * 1000) as f64; // in ms
    move || performance.now() - start < timeout
}

fn compute_prime<F>(mut within_time: F, mn: u128, sqrt: u128) -> f64
where
    F: FnMut() -> bool,
{
    for i in 2..sqrt {
        if mn % i == 0 {
            return -1.0;
        } // is prime
        if !within_time() {
            return i as f64 / sqrt as f64; // time ended, fraction tested
        }
    }
    1.0
}

fn compute_benchmark<F>(mut within_time: F) -> f64
where
    F: FnMut() -> bool,
{
    let mut if_fractional: f64 = 0.0;
    while within_time() {
        let mn: u128 = mersenne();
        let sqrt: u128 = (mn as f64).sqrt() as u128;
        if_fractional = compute_prime(&mut within_time, mn, sqrt);
    }
    let score = next_exponent() as f64 + if_fractional;
    score
}


#[cfg(not(target_arch = "wasm32"))]
pub fn cpu_mt() -> String {
    let mut timers: Vec<Box<dyn FnMut() -> bool + Send + 'static>> =
        Vec::with_capacity(MAX_THREADS);
    for _ in 0..MAX_THREADS { timers.push(Box::new(remaining_timer())); }

    let total_score = Arc::new(Mutex::new(vec![0.0; MAX_THREADS]));
    let mut mt_handles = vec![];
    
    for thread_id in 0..MAX_THREADS {
        let mut timer = timers.remove(0);
        let total_score = Arc::clone(&total_score);
        
        let handle = thread::spawn(move || {
            let mut scores = total_score.lock().unwrap();
            let score = compute_benchmark(&mut timer);
            scores[thread_id] = score;
        });

        mt_handles.push(handle);
    }

    for handle in mt_handles { handle.join().unwrap(); }

    let sum_score= total_score.lock().unwrap().iter().sum::<f64>();
    let fractional_avg: f64 = sum_score / MAX_THREADS as f64;
    let score = (next_exponent() - (MAX_THREADS / 2) as u32) as f64 + fractional_avg;
    score.to_string()
}


#[cfg(target_arch = "wasm32")]
pub fn cpu_mt() -> String {
    let mut timer = remaining_timer();
    let score = compute_benchmark(&mut timer);
    score.to_string()
}
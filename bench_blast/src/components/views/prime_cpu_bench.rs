const TIME_LIMIT_SECS: u64 = 10;
const BASE : u128 = 2; 
static SAFE_COUNTER : std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(2);


fn next_exponent() -> u32 {
    SAFE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn mersenne() -> u128 {
    BASE.pow(next_exponent()) -1
}

fn compute_prime<F>(mut within_time: F, mn: u128, sqrt: u128) -> f64 
where
    F: FnMut() -> bool,
{
    for i in 2..sqrt {
        if mn % i == 0 { return -1.0 ;} // is prime
        if !within_time() {
            return i as f64 / sqrt as f64; // time ended, fraction tested
        }
    }
    1.0
}



fn compute_benchmark<F>(mut within_time: F) -> String
where
    F: FnMut() -> bool,
{

    let mut if_fractional: f64 = 0.0;

    while within_time() {
        let mn: u128 = mersenne();
        let sqrt: u128 = (mn as f64).sqrt() as u128;
        if_fractional = compute_prime(&mut within_time, mn , sqrt) ;
    }

    let score  = (next_exponent())  as f64 + if_fractional;
    score.to_string()
}




pub fn cpu() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(TIME_LIMIT_SECS);
        compute_benchmark(|| start.elapsed() < timeout)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().expect("no global `window` exists");
        let performance = window.performance().expect("performance should be available");
        let start = performance.now();
        let timeout = (TIME_LIMIT_SECS * 1000) as f64;// 15 seconds in milliseconds
        compute_benchmark(|| performance.now() - start < timeout)
    }
}
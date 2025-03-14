use std::thread;
use std::time::Duration;

pub fn run_async<F>(func: F)
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(func);
}

pub fn delay(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

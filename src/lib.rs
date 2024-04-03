use std::sync::Mutex;
use std::time::{Duration, SystemTime};

struct Throttle {
    timeout: Duration,
    threshold: usize,
    last_time: SystemTime,
    count: usize,
}

impl Throttle {
    pub fn new(timeout: Duration, threshold: usize) -> Mutex<Self> {
        Mutex::new(Throttle {
            timeout,
            threshold,
            last_time: SystemTime::UNIX_EPOCH,
            count: 0,
        })
    }

    /// Return true if not over the rate limit, otherwise return false.
    fn accept(&mut self) -> bool {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(self.last_time) {
            if duration > self.timeout {
                self.count = 0;
                self.last_time = now;
            }
        } else {
            return false;
        }
        if self.count < self.threshold {
            self.count = self.count + 1;
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicIsize, Ordering};
    use std::thread;
    use std::thread::sleep;
    use super::*;

    #[test]
    fn it_works() {
        let throttle = Arc::new(Throttle::new(Duration::from_secs(1), 22));
        let counter = Arc::new(AtomicIsize::new(0));
        for _ in 0..100 {
            let throttle = Arc::clone(&throttle);
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                if let Ok(mut throttle) = throttle.lock() {
                    if throttle.accept() {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
        assert_eq!(counter.load(Ordering::Relaxed), 22);
        sleep(Duration::from_secs(1));
        if let Ok(mut throttle) = throttle.lock() {
            if throttle.accept() {
                counter.fetch_add(2, Ordering::Relaxed);
            }
        }
        assert_eq!(counter.load(Ordering::Relaxed), 24);
    }
}

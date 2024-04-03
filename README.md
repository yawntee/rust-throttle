# rust-throttle
rate limit library

```rust
use once_cell::sync::Lazy;

static THROTTLE: Lazy<Mutex<Throttle>> = Lazy::new(|| Mutex::new(Throttle::new(Duration::from_secs(1), 10)));

fn main(){
    if let Ok(mut throttle) = THROTTLE.lock() {
        if throttle.accept() {
            //will run if number of calls less than 10 times duration 1s
        }
    }
}
```

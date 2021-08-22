# calculagraph
A handy library for measuring the execution time of function.

## Usage
```toml
[dependencies]
calculagraph = "0.1"
```
```rust
use std::{thread, time};
use calculagraph::timer_println;

#[timer_println(ms)]
fn main() {
    thread::sleep(time::Duration::from_millis(10));
    println!("job done");
}
```

## More
[https://docs.rs/calculagraph](https://docs.rs/calculagraph)


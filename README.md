# calculagraph
A handy library for measuring the execution time of function.

[![Package][package-img]][package-url]

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
The above example will print `fn:main cost 10ms` at the end, You can also use the second
argument to define the format string you need.

## More
[https://docs.rs/calculagraph](https://docs.rs/calculagraph)


[package-url]: https://crates.io/crates/calculagraph
[package-img]: https://img.shields.io/crates/v/calculagraph.svg
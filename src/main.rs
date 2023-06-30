#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod config;
mod cli;

fn main() {
    println!("Hello, world!");
}

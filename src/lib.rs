mod closures;
mod errors;
mod generics;
pub mod grep;
mod lifetimes;
pub mod misc;
mod traits;

pub fn run() {
    closures::run();
    grep::run();
    lifetimes::run();
    traits::run();
    generics::run();
    errors::run();
    misc::run();
}

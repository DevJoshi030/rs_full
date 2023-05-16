mod errors;
mod generics;
mod lifetimes;
pub mod misc;
mod traits;

pub fn run() {
    misc::run();
    errors::run();
    generics::run();
    traits::run();
    lifetimes::run();
}

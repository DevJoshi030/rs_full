mod closures;
mod errors;
mod generics;
pub mod grep;
mod iterators;
mod lifetimes;
pub mod misc;
mod smart_pointers;
mod smart_pointers_2;
mod threads;
mod traits;

pub fn run() {
    threads::run();
    smart_pointers_2::run();
    smart_pointers::run();
    iterators::run();
    closures::run();
    grep::run();
    lifetimes::run();
    traits::run();
    generics::run();
    errors::run();
    misc::run();
}

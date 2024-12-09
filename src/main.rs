mod A;
mod B;
mod C;
mod D;
mod E;
mod F;
mod G;
mod I;

use std::{fs, thread};
use std::time::Instant;
use crate::I::run;

fn main() {
    run();
    /*let builder = thread::Builder::new()
        .name("kasiopea".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(|| {
        // stack-intensive operations
    }).unwrap();

    handler.join().unwrap();*/
}

// by default the stack unwinding will only give something like this:
//     thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// Though there is the reason of panic and the line of the code is showing where the panic has occured, sometimes we want to get more info about the call stack.


// ## FILL in the blank to display the whole call stack
// ## Tips: you can find the clue in the default panic info 
// $ __ cargo run

// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `[97, 98, 99]`,
//  right: `[96, 97, 98]`', src/main.rs:3:5
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
//    1: core::panicking::panic_fmt
//              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
//    2: core::panicking::assert_failed_inner
//    3: core::panicking::assert_failed
//              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:154:5
//    4: study_cargo::main
//              at ./src/main.rs:3:5
//    5: core::ops::function::FnOnce::call_once
//              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

// unwinding and abort
// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
// But this walk back and clean up is a lot of work. The alternative is to immediately abort the program without cleaning up.
// If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting by adding below content to Cargo.toml:
// [profile.release]
// panic = 'abort'

fn main() {
    println!("Hello, world!");
}

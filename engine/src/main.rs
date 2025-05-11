use std::env;

mod bound;
mod config;
mod eval;
mod logger;
mod nnue;
mod search;
mod time_control;
mod uci;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use uci::UciReader;

    let args: Vec<String> = env::args().collect();
    UciReader::default().run(args);
}

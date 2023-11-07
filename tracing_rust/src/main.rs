use tracing_rust::{addition, init_tracing_basic};

fn main() {
    init_tracing_basic();

    let ret = addition(1, 2);
    println!("1 + 2 = {ret}")
}

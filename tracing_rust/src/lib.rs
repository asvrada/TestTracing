use std::ffi::c_int;
use std::fs::File;

use tracing::instrument;
use tracing_subscriber::EnvFilter;

#[no_mangle]
#[instrument]
pub extern "C" fn addition(a: c_int, b: c_int) -> c_int {
    tracing::info!("Running addition");
    tracing::error!("Test logging errors");

    let result = a + b;

    println!("addition({a},{b}) = {result}");
    tracing::trace!(a, b, result, "addition result");
    result
}

#[no_mangle]
pub extern "C" fn init_tracing_basic() {
    // Basic env filter
    let filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

#[no_mangle]
pub extern "C" fn init_tracing_file() {
    // Write to file
    let filter = EnvFilter::from_default_env();

    let log_file = File::create("log.txt").expect("Failed to create file");
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(log_file)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_addition() {
        let result = addition(2, 2);
        assert_eq!(result, 4);
    }
}

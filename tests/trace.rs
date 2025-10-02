#![no_std]
#![no_main]

use defmt_rtt as _;
use hal as _;
use panic_probe as _;

#[mantra_rust_macros::req("REQ-1")]
fn some_fn() -> u32 {
    42
}

#[defmt_test::tests]
mod tests {
    use crate::some_fn;

    #[test]
    fn pass() {
        defmt::assert_eq!(42, some_fn());
    }
}

#![no_main]
use libfuzzer_sys::fuzz_target;

use case_conv::to_uppercase;

fuzz_target!(|data: &str| {
    let upper = to_uppercase(data);
    assert_eq!(upper, data.to_uppercase());
});

#![no_main]
use libfuzzer_sys::fuzz_target;

use case_conv::to_lowercase;

fuzz_target!(|data: &str| {
    let lower = to_lowercase(data);
    assert_eq!(lower, data.to_lowercase());
});

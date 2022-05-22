#![no_main]
use libfuzzer_sys::fuzz_target;

use case_conv::is_ascii;

fuzz_target!(|data: &str| {
    let is_ascii = is_ascii(data.as_bytes());
    assert_eq!(is_ascii, data.is_ascii());
});

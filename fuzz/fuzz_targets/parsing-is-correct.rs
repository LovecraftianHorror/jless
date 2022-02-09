#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let serde_parsed = serde_json::from_str::<serde_json::Value>(s).is_ok();
        let jless_parsed = jless::jsonparser::parse(s.to_owned()).is_ok();

        assert!(serde_parsed == jless_parsed);
    }
});

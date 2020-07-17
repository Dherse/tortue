#![no_main]
use libfuzzer_sys::fuzz_target;

use tortue_bencode::{ parse, writer::write};

fuzz_target!(|data: &[u8]| {
    if let Ok((_r, parsed)) = parse(data) {
        let mut bytes = Vec::with_capacity(data.len());
        assert!(write(&parsed, &mut bytes).is_ok());

        let parsed_again = parse(&bytes[..]);
        assert!(parsed_again.is_ok());

        assert_eq!(parsed, parsed_again.unwrap().1);
    }
});

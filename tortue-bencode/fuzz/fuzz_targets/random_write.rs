#![no_main]
use libfuzzer_sys::fuzz_target;

use tortue_bencode::{ BencodedValue, parse, writer::write };

fuzz_target!(|data: BencodedValue| {
    let mut bytes = vec![];
    assert!(write(&data, &mut bytes).is_ok());

    let parsed = parse(&bytes);
    assert!(parsed.is_ok());

    assert_eq!(parsed.unwrap().1, data);

    std::mem::drop(data);
});

use super::*;

extern crate test;

#[bench]
fn decode_empty(b: &mut test::Bencher) {
    b.iter(|| {
        let mut out = [];
        test::black_box(decode(Standard, b"", &mut out).unwrap());
    });
}

const SMALL_TEST: &[u8] = b"TGlmZeKAmXMgYnV0IGEgd2Fsa2luZyBzaGFkb3csIGEgcG9vciBwbGF5ZXIKVGhhdCBzdHJ1dHMgYW5kIGZyZXRzIGhpcyBob3VyIHVwb24gdGhlIHN0YWdlLApBbmQgdGhlbiBpcyBoZWFyZCBubyBtb3JlLiBJdCBpcyBhIHRhbGUKVG9sZCBieSBhbiBpZGlvdCwgZnVsbCBvZiBzb3VuZCBhbmQgZnVyeSwKU2lnbmlmeWluZyBub3RoaW5nLg==";
const SMALL_TEST_LEN: usize = decode_len_estimate(SMALL_TEST.len());
const LARGE_TEST: &[u8] = include_bytes!("data/hamlet.b64.txt");
const LARGE_TEST_LEN: usize = decode_len_estimate(LARGE_TEST.len());

#[bench]
fn decode_small(b: &mut test::Bencher) {
    b.iter(|| {
        let mut out = [0u8; SMALL_TEST_LEN];
        test::black_box(decode(Standard, SMALL_TEST, &mut out).unwrap());
    });
}

#[bench]
fn decode_large(b: &mut test::Bencher) {
    b.iter(|| {
        let mut out = [0u8; LARGE_TEST_LEN];
        test::black_box(decode(Standard, LARGE_TEST, &mut out).unwrap());
    });
}

mod baseline {
    use super::{test, LARGE_TEST, LARGE_TEST_LEN, SMALL_TEST, SMALL_TEST_LEN};
    use base64::Engine;

    #[bench]
    fn decode_empty(b: &mut test::Bencher) {
        b.iter(|| {
            let mut out = [];
            test::black_box(
                base64::engine::general_purpose::STANDARD
                    .decode_slice(b"", &mut out)
                    .unwrap(),
            );
        });
    }

    #[bench]
    fn decode_small(b: &mut test::Bencher) {
        b.iter(|| {
            let mut out = [0u8; SMALL_TEST_LEN];
            test::black_box(
                base64::engine::general_purpose::STANDARD
                    .decode_slice(SMALL_TEST, &mut out)
                    .unwrap(),
            );
        });
    }

    #[bench]
    fn decode_large(b: &mut test::Bencher) {
        b.iter(|| {
            let mut out = [0u8; LARGE_TEST_LEN];
            test::black_box(
                base64::engine::general_purpose::STANDARD
                    .decode_slice(LARGE_TEST, &mut out)
                    .unwrap(),
            );
        });
    }
}

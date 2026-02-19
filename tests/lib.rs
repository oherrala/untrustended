#![deny(warnings)]

use core::net::{Ipv4Addr, Ipv6Addr};

use quickcheck::quickcheck;

use untrusted::{Input, Reader};
use untrustended::ReaderExt;

#[inline]
fn reader(buf: &'_ [u8]) -> Reader<'_> {
    Reader::new(Input::from(buf))
}

#[inline]
fn builder(buf: &mut [u8]) -> pktbuilder::Builder<'_> {
    pktbuilder::Builder::new(buf)
}

quickcheck! {
    fn prop_read_u8(xs: u8) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_byte(xs).expect("add_byte");
        let mut reader = reader(&buf);
        xs == reader.read_u8().expect("read_u8")
    }

    fn prop_read_u16be(xs: u16) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u16_be(xs).expect("add_u16_be");
        let mut reader = reader(&buf);
        xs == reader.read_u16be().expect("read_u16be")
    }

    fn prop_read_u24be(xs: u32) -> bool {
        let xs = xs >> 8;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add(&xs.to_be_bytes()[1..4]).expect("add");
        let mut reader = reader(&buf);
        xs == reader.read_u24be().expect("read_u24be")
    }

    fn prop_read_u32be(xs: u32) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_be(xs).expect("add_u32_be");
        let mut reader = reader(&buf);
        xs == reader.read_u32be().expect("read_u32be")
    }

    fn prop_read_u48be(xs: u64) -> bool {
        let xs = xs >> 16;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add(&xs.to_be_bytes()[2..8]).expect("add");
        let mut reader = reader(&buf);
        xs == reader.read_u48be().expect("read_u48be")
    }

    fn prop_read_u64be(xs: u64) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_be(xs).expect("add_u64_be");
        let mut reader = reader(&buf);
        xs == reader.read_u64be().expect("read_u64be")
    }

    fn prop_read_u128be(xs: u128) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u128_be(xs).expect("add_u128_be");
        let mut reader = reader(&buf);
        xs == reader.read_u128be().expect("read_u128be")
    }

   fn prop_read_u16le(xs: u16) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u16_le(xs).expect("add_u16_le");
        let mut reader = reader(&buf);
        xs == reader.read_u16le().expect("read_u16le")
    }

    fn prop_read_u24le(xs: u32) -> bool {
        let xs = xs >> 8;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_le(xs).expect("add_u32_le");
        let mut reader = reader(&buf);
        xs == reader.read_u24le().expect("read_u24le")
    }

    fn prop_read_u32le(xs: u32) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_le(xs).expect("add_u32_le");
        let mut reader = reader(&buf);
        xs == reader.read_u32le().expect("read_u32le")
    }

    fn prop_read_u48le(xs: u64) -> bool {
        let xs = xs >> 16;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_le(xs).expect("add_u64_le");
        let mut reader = reader(&buf);
        xs == reader.read_u48le().expect("read_u48le")
    }

    fn prop_read_u64le(xs: u64) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_le(xs).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_u64le().expect("read_u64le")
    }

    fn prop_read_u128le(xs: u128) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u128_le(xs).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_u128le().expect("read_u128le")
    }


    fn prop_read_i16be(xs: i16) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u16_be(xs as u16).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i16be().expect("read_i16be")
    }

    fn prop_read_i24be(xs: i32) -> bool {
        let xs = xs >> 8;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add(&xs.to_be_bytes()[1..4]).expect("add");
        let mut reader = reader(&buf);
        xs == reader.read_i24be().expect("read_i24be")
    }

    fn prop_read_i32be(xs: i32) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_be(xs as u32).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i32be().expect("read_i32be")
    }

    fn prop_read_i48be(xs: i64) -> bool {
        let xs = xs >> 16;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add(&xs.to_be_bytes()[2..8]).expect("add");
        let mut reader = reader(&buf);
        xs == reader.read_i48be().expect("read_u48be")
    }

    fn prop_read_i64be(xs: i64) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_be(xs as u64).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i64be().expect("read_i64be")
    }

    fn prop_read_i128be(xs: i128) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u128_be(xs as u128).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i128be().expect("read_i128be")
    }

   fn prop_read_i16le(xs: i16) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u16_le(xs as u16).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i16le().expect("read_i16le")
    }

    fn prop_read_i24le(xs: i32) -> bool {
        let xs = xs >> 8;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_le(xs as u32).expect("add_u32_le");
        let mut reader = reader(&buf);
        xs == reader.read_i24le().expect("read_i24le")
    }

    fn prop_read_i32le(xs: i32) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u32_le(xs as u32).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i32le().expect("read_i32le")
    }

    fn prop_read_i48le(xs: i64) -> bool {
        let xs = xs >> 16;
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_le(xs as u64).expect("add_u64_be");
        let mut reader = reader(&buf);
        xs == reader.read_i48le().expect("read_i48le")
    }

    fn prop_read_i64le(xs: i64) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u64_le(xs as u64).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i64le().expect("read_i64le")
    }

    fn prop_read_i128le(xs: i128) -> bool {
        let mut buf = vec![0xCD; 16];
        builder(&mut buf).add_u128_le(xs as u128).expect("write_byte");
        let mut reader = reader(&buf);
        xs == reader.read_i128le().expect("read_i128le")
    }

    fn prop_read_bytes_less_safe(xs: Vec<u8>) -> bool {
        let mut reader = reader(&xs);
        xs == reader.read_bytes_less_safe(xs.len()).expect("read_bytes_less_safes")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_utf8(xs: String) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(xs.as_bytes()).expect("write_all");
        let len = buf.len();
        let mut reader = reader(&buf);
        xs == reader.read_utf8(len).expect("read_utf8")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_boxu8_with_fromreader(xs: Vec<u8>) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(&xs).expect("write_all");
        let mut reader = reader(&buf);
        let xs = Box::from(xs);
        xs == reader.read_be::<Box<[u8]>>().expect("read_be")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_vecu8_with_fromreader(xs: Vec<u8>) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(&xs).expect("write_all");
        let mut reader = reader(&buf);
        xs == reader.read_be::<Vec<u8>>().expect("read_be")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_boxstr_with_fromreader(xs: String) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(xs.as_bytes()).expect("write_all");
        let mut reader = reader(&buf);
        let xs = xs.into_boxed_str();
        xs == reader.read_be::<Box<str>>().expect("read_be")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_string_with_fromreader(xs: String) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(xs.as_bytes()).expect("write_all");
        let mut reader = reader(&buf);
        xs == reader.read_be::<String>().expect("read_be")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_utf16(xs: String) -> bool {
        let mut buf = vec![0xCD; 1500];
        if xs.is_ascii() || xs.len() > buf.len() {
            return true;
        }
        let mut builder = builder(&mut buf);
        let mut len = 0;
        for short in xs.encode_utf16() {
            builder.add_u16_be(short).expect("add_u16_be");
            len += 2;
        }
        let mut reader = reader(&buf);
        xs == reader.read_utf16(len).expect("read_utf16")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_cstring(xs: std::ffi::CString) -> bool {
        let mut reader = reader(xs.as_bytes_with_nul());
        xs == reader.read_cstring(usize::MAX).expect("read_cstring")
    }

    #[cfg(feature = "use_std")]
    fn prop_read_multiple_cstrings(xs: Vec<std::ffi::CString>) -> bool {
        use std::io::Write;
        let mut buf = Vec::new();
        for x in &xs {
            buf.write_all(x.as_bytes_with_nul()).expect("write_all");
        }
        let mut reader = reader(&buf);
        for x in &xs {
            if *x != reader.read_cstring(usize::MAX).expect("read_cstring") {
                return false;
            }
        }
        true
    }
}

#[test]
fn read_i8_specials() {
    let specials: &[i8] = &[i8::MIN, i8::MIN + 1, -1, 0, 1, i8::MAX - 1, i8::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_byte(*s as u8).expect("add_byte");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i8().expect("read_i8"));
    }
}

#[test]
fn read_i16be_specials() {
    let specials: &[i16] = &[i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX - 1, i16::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u16_be(*s as u16).expect("add_u16_be");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i16be().expect("read_i16be"));
    }
}

#[test]
fn read_i24be_specials() {
    let specials: &[i32] = &[-8_388_608, -8_388_607, -1, 0, 1, 8_388_606, 8_388_607];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add(&s.to_be_bytes()[1..4]).expect("add");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i24be().expect("read_i24be"));
    }
}

#[test]
fn read_i32be_specials() {
    let specials: &[i32] = &[i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u32_be(*s as u32).expect("add_u32_be");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i32be().expect("read_i32be"));
    }
}

#[test]
fn read_i48be_specials() {
    let specials: &[i64] = &[
        -140_737_488_355_328,
        -140_737_488_355_327,
        -1,
        0,
        1,
        140_737_488_355_326,
        140_737_488_355_327,
    ];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add(&s.to_be_bytes()[2..8]).expect("add");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i48be().expect("read_i48be"));
    }
}

#[test]
fn read_i64be_specials() {
    let specials: &[i64] = &[i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u64_be(*s as u64).expect("add_u64_be");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i64be().expect("read_i64be"));
    }
}

#[test]
fn read_i16le_specials() {
    let specials: &[i16] = &[i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX - 1, i16::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u16_le(*s as u16).expect("add_u16_le");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i16le().expect("read_i16le"));
    }
}

#[test]
fn read_i24le_specials() {
    let specials: &[i32] = &[-8_388_608, -8_388_607, -1, 0, 1, 8_388_606, 8_388_607];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u32_le(*s as u32).expect("add_u32_le");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i24le().expect("read_i24le"));
    }
}

#[test]
fn read_i32le_specials() {
    let specials: &[i32] = &[i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX];

    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u32_le(*s as u32).expect("add_u32_le");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i32le().expect("read_i32le"));
    }
}

#[test]
fn read_i48le_specials() {
    let specials: &[i64] = &[
        -140_737_488_355_328,
        -140_737_488_355_327,
        -1,
        0,
        1,
        140_737_488_355_326,
        140_737_488_355_327,
    ];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u64_le(*s as u64).expect("add");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i48le().expect("read_i48le"));
    }
}

#[test]
fn read_i64le_specials() {
    let specials: &[i64] = &[i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
    for s in specials {
        let mut buf = [0xCD; 16];
        builder(&mut buf).add_u64_le(*s as u64).expect("add_u64_le");
        let mut reader = reader(&buf);
        assert_eq!(*s, reader.read_i64le().expect("read_i64le"));
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_utf16_with_odd_length() {
    let mut reader = reader(&[]);
    match reader.read_utf16(3) {
        Err(err) => assert_eq!(err, untrustended::Error::ParseError),
        _ => panic!("Test shouldn't reach here"),
    }
}

#[test]
fn read_ipv4addr() {
    use std::io::Write;
    let addrs: &[Ipv4Addr] = &[
        "0.0.0.0".parse().expect("parse ipv4 addr"),
        "192.0.2.1".parse().expect("parse ipv4 addr"),
        "198.51.100.128".parse().expect("parse ipv4 addr"),
        "203.0.113.255".parse().expect("parse ipv4 addr"),
        "255.255.255.255".parse().expect("parse ipv4 addr"),
    ];
    for addr in addrs {
        let mut buf = Vec::new();
        buf.write_all(&addr.octets()).expect("write_all");
        let mut reader = reader(&buf);
        assert_eq!(*addr, reader.read_ipv4addr().expect("read_ipv4addr"));
    }
}

#[test]
fn read_ipv6addr() {
    use std::io::Write;
    let addrs: &[Ipv6Addr] = &[
        "2001:DB8::".parse().expect("parse ipv6 addr"),
        "2001:DB8:ff00:00ff:f00f:0ff0:0000:ffff"
            .parse()
            .expect("parse ipv6 addr"),
        "2001:DB8:ffff:ffff:ffff:ffff:ffff:ffff"
            .parse()
            .expect("parse ipv6 addr"),
    ];
    for addr in addrs {
        let mut buf = Vec::new();
        buf.write_all(&addr.octets()).expect("write_all");
        let mut reader = reader(&buf);
        assert_eq!(*addr, reader.read_ipv6addr().expect("read_ipv6addr"));
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_cstring() {
    use std::ffi::CStr;
    const TESTS: &[&CStr] = &[c"", c"a", c"aaaaaaa", c"Hello World!"];

    for test in TESTS {
        let mut reader = reader(test.to_bytes_with_nul());
        let read = reader.read_cstring(usize::MAX).expect("read_cstring");
        assert_eq!(*test, read.as_c_str());
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_cstring_invalid_string() {
    const TESTS: &[&[u8]] = &[b"", b"a", b"aaaaaaa", b"Hello World!"];

    for test in TESTS {
        let mut reader = reader(test);
        assert!(
            reader.read_cstring(usize::MAX).is_err(),
            "Invalid CString {test:?}"
        );
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_cstring_missing_null() {
    const TESTS: &[&[u8]] = &[b"aaaaaaa", b"Hello World!"];

    for test in TESTS {
        let mut reader = reader(test);
        assert!(reader.read_cstring(6).is_err(), "Invalid CString {test:?}");
    }
}

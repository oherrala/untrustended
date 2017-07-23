extern crate byteorder;
#[macro_use]
extern crate quickcheck;

extern crate untrusted;
extern crate untrustended;

use untrusted::{Input, Reader};
use untrustended::{ReaderExt, Error};

use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian, LittleEndian};

#[inline]
fn reader(buf: &[u8]) -> Reader {
    Reader::new(Input::from(buf))
}

quickcheck! {
    fn prop_read_u8(xs: u8) -> bool {
        let mut buf = Vec::new();
        buf.write_u8(xs).expect("write_u8");
        let mut reader = reader(&buf);
        xs == reader.read_u8().expect("read_u8")
    }

    fn prop_read_u16be(xs: u16) -> bool {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(xs).expect("write_u16");
        let mut reader = reader(&buf);
        xs == reader.read_u16be().expect("read_u16be")
    }

    fn prop_read_u32be(xs: u32) -> bool {
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(xs).expect("write_u32");
        let mut reader = reader(&buf);
        xs == reader.read_u32be().expect("read_u32be")
    }

    fn prop_read_u64be(xs: u64) -> bool {
        let mut buf = Vec::new();
        buf.write_u64::<BigEndian>(xs).expect("write_u64");
        let mut reader = reader(&buf);
        xs == reader.read_u64be().expect("read_u64be")
    }

    fn prop_read_u16le(xs: u16) -> bool {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>(xs).expect("write_u16");
        let mut reader = reader(&buf);
        xs == reader.read_u16le().expect("read_u16le")
    }

    fn prop_read_u32le(xs: u32) -> bool {
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(xs).expect("write_u32");
        let mut reader = reader(&buf);
        xs == reader.read_u32le().expect("read_u32le")
    }

    fn prop_read_u64le(xs: u64) -> bool {
        let mut buf = Vec::new();
        buf.write_u64::<LittleEndian>(xs).expect("write_u64");
        let mut reader = reader(&buf);
        xs == reader.read_u64le().expect("read_u64le")
    }

    fn prop_read_i16be(xs: i16) -> bool {
        let mut buf = Vec::new();
        buf.write_i16::<BigEndian>(xs).expect("write_i16");
        let mut reader = reader(&buf);
        xs == reader.read_i16be().expect("read_i16be")
    }

   fn prop_read_i32be(xs: i32) -> bool {
        let mut buf = Vec::new();
        buf.write_i32::<BigEndian>(xs).expect("write_i32");
        let mut reader = reader(&buf);
        xs == reader.read_i32be().expect("read_i32be")
    }

    fn prop_read_i64be(xs: i64) -> bool {
        let mut buf = Vec::new();
        buf.write_i64::<BigEndian>(xs).expect("write_i64");
        let mut reader = reader(&buf);
        xs == reader.read_i64be().expect("read_i64be")
    }

    fn prop_read_i16le(xs: i16) -> bool {
        let mut buf = Vec::new();
        buf.write_i16::<LittleEndian>(xs).expect("write_i16");
        let mut reader = reader(&buf);
        xs == reader.read_i16le().expect("read_i16le")
    }

    fn prop_read_i32le(xs: i32) -> bool {
        let mut buf = Vec::new();
        buf.write_i32::<LittleEndian>(xs).expect("write_i32");
        let mut reader = reader(&buf);
        xs == reader.read_i32le().expect("read_i32le")
    }

    fn prop_read_i64le(xs: i64) -> bool {
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(xs).expect("write_i64");
        let mut reader = reader(&buf);
        xs == reader.read_i64le().expect("read_i64le")
    }

    fn prop_read_bytes(xs: Vec<u8>) -> bool {
        let mut buf = Vec::new();
        buf.write_all(&xs).expect("write_all");
        let len = buf.len();
        let mut reader = reader(&buf);
        xs == reader.read_bytes(len).expect("read_bytes")
    }

    fn prop_read_utf8(xs: String) -> bool {
        let mut buf = Vec::new();
        buf.write_all(xs.as_bytes()).expect("write_all");
        let len = buf.len();
        let mut reader = reader(&buf);
        xs == reader.read_utf8(len).expect("read_utf8")
    }

    fn prop_read_utf16(xs: String) -> bool {
        let mut buf = Vec::new();
        for short in xs.encode_utf16() {
            buf.write_u16::<BigEndian>(short).expect("write_u16");
        }
        let len = buf.len();
        let mut reader = reader(&buf);
        xs == reader.read_utf16(len).expect("read_utf16")
    }
}

#[test]
fn read_i16be_specials() {
    let specials = vec![
        i16::min_value(),
        i16::min_value() + 1,
        -1,
        0,
        1,
        i16::max_value() - 1,
        i16::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i16::<BigEndian>(s).expect("write_i16");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i16be().expect("read_i16be"));
    }
}

#[test]
fn read_i32be_specials() {
    let specials = vec![
        i32::min_value(),
        i32::min_value() + 1,
        -1,
        0,
        1,
        i32::max_value() - 1,
        i32::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i32::<BigEndian>(s).expect("write_i32");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i32be().expect("read_i32be"));
    }
}

#[test]
fn read_i64be_specials() {
    let specials = vec![
        i64::min_value(),
        i64::min_value() + 1,
        -1,
        0,
        1,
        i64::max_value() - 1,
        i64::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i64::<BigEndian>(s).expect("write_i64");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i64be().expect("read_i64be"));
    }
}


#[test]
fn read_i16le_specials() {
    let specials = vec![
        i16::min_value(),
        i16::min_value() + 1,
        -1,
        0,
        1,
        i16::max_value() - 1,
        i16::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i16::<LittleEndian>(s).expect("write_i16");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i16le().expect("read_i16le"));
    }
}

#[test]
fn read_i32le_specials() {
    let specials = vec![
        i32::min_value(),
        i32::min_value() + 1,
        -1,
        0,
        1,
        i32::max_value() - 1,
        i32::max_value(),
    ];

    for s in specials {
        let mut buf = Vec::new();
        buf.write_i32::<LittleEndian>(s).expect("write_i32");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i32le().expect("read_i32le"));
    }
}

#[test]
fn read_i64le_specials() {
    let specials = vec![
        i64::min_value(),
        i64::min_value() + 1,
        -1,
        0,
        1,
        i64::max_value() - 1,
        i64::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(s).expect("write_i64");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i64le().expect("read_i64le"));
    }
}

#[test]
fn read_utf16_with_odd_length() {
    let mut reader = reader(&[]);
    match reader.read_utf16(3) {
        Err(err) => assert_eq!(err, Error::ParseError),
        _ => panic!("Test shouldn't reach here"),
    }
}

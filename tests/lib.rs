#![deny(warnings)]

use quickcheck::quickcheck;

use untrusted::{Input, Reader};
use untrustended::ReaderExt;

use byteorder::{WriteBytesExt, BigEndian, LittleEndian};

#[inline]
fn reader<'r>(buf: &'r [u8]) -> Reader<'r> {
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

    fn prop_read_u24be(xs: (u8, u8, u8)) -> bool {
        let mut buf = Vec::new();
        let ys = (u32::from(xs.0) << 16) + (u32::from(xs.1) << 8) + u32::from(xs.2);
        buf.write_u24::<BigEndian>(ys).expect("write_u24");
        let mut reader = reader(&buf);
        ys == reader.read_u24be().expect("read_u24be")
    }

    fn prop_read_u32be(xs: u32) -> bool {
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(xs).expect("write_u32");
        let mut reader = reader(&buf);
        xs == reader.read_u32be().expect("read_u32be")
    }

    fn prop_read_u48be(xs: (u16, u16, u16)) -> bool {
        let mut buf = Vec::new();
        let ys = (u64::from(xs.0) << 32) + (u64::from(xs.1) << 16) + u64::from(xs.2);
        buf.write_u16::<BigEndian>(xs.0).expect("write_u16");
        buf.write_u16::<BigEndian>(xs.1).expect("write_u16");
        buf.write_u16::<BigEndian>(xs.2).expect("write_u16");
        let mut reader = reader(&buf);
        ys == reader.read_u48be().expect("read_u48be")
    }

    fn prop_read_u64be(xs: u64) -> bool {
        let mut buf = Vec::new();
        buf.write_u64::<BigEndian>(xs).expect("write_u64");
        let mut reader = reader(&buf);
        xs == reader.read_u64be().expect("read_u64be")
    }

    fn prop_read_u128be(xs: (u64, u64)) -> bool {
        let mut buf = Vec::new();
        let ys = (u128::from(xs.0) << 64) + u128::from(xs.1);
        buf.write_u64::<BigEndian>(xs.0).expect("write_u64");
        buf.write_u64::<BigEndian>(xs.1).expect("write_u64");
        let mut reader = reader(&buf);
        ys == reader.read_u128be().expect("read_u128be")
    }

    fn prop_read_u16le(xs: u16) -> bool {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>(xs).expect("write_u16");
        let mut reader = reader(&buf);
        xs == reader.read_u16le().expect("read_u16le")
    }

    fn prop_read_u24le(xs: (u8, u8, u8)) -> bool {
        let mut buf = Vec::new();
        let ys = (u32::from(xs.0) << 16) + (u32::from(xs.1) << 8) + u32::from(xs.2);
        buf.write_u24::<LittleEndian>(ys).expect("write_u24");
        let mut reader = reader(&buf);
        ys == reader.read_u24le().expect("read_u24le")
    }

    fn prop_read_u32le(xs: u32) -> bool {
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(xs).expect("write_u32");
        let mut reader = reader(&buf);
        xs == reader.read_u32le().expect("read_u32le")
    }

    fn prop_read_u48le(xs: (u16, u16, u16)) -> bool {
        let mut buf = Vec::new();
        let ys = (u64::from(xs.0) << 32) + (u64::from(xs.1) << 16) + u64::from(xs.2);
        buf.write_u16::<LittleEndian>(xs.2).expect("write_u16");
        buf.write_u16::<LittleEndian>(xs.1).expect("write_u16");
        buf.write_u16::<LittleEndian>(xs.0).expect("write_u16");
        let mut reader = reader(&buf);
        ys == reader.read_u48le().expect("read_u48le")
    }

    fn prop_read_u64le(xs: u64) -> bool {
        let mut buf = Vec::new();
        buf.write_u64::<LittleEndian>(xs).expect("write_u64");
        let mut reader = reader(&buf);
        xs == reader.read_u64le().expect("read_u64le")
    }

    fn prop_read_u128le(xs: (u64, u64)) -> bool {
        let mut buf = Vec::new();
        let ys = (u128::from(xs.0) << 64) + u128::from(xs.1);
        buf.write_u64::<LittleEndian>(xs.1).expect("write_u64");
        buf.write_u64::<LittleEndian>(xs.0).expect("write_u64");
        let mut reader = reader(&buf);
        ys == reader.read_u128le().expect("read_u128le")
    }

    fn prop_read_i8(xs: i8) -> bool {
        let mut buf = Vec::new();
        buf.write_i8(xs).expect("write_i8");
        let mut reader = reader(&buf);
        xs == reader.read_i8().expect("read_i8")
    }

    fn prop_read_i16be(xs: i16) -> bool {
        let mut buf = Vec::new();
        buf.write_i16::<BigEndian>(xs).expect("write_i16");
        let mut reader = reader(&buf);
        xs == reader.read_i16be().expect("read_i16be")
    }

    fn prop_read_i24be(xs: (u8, u8, u8)) -> bool {
        let mut buf = Vec::new();
        let ys = (i32::from(xs.0) << 16) + (i32::from(xs.1) << 8) + i32::from(xs.2);
        buf.write_i24::<BigEndian>(ys).expect("write_i24");
        let mut reader = reader(&buf);
        ys == reader.read_i24be().expect("read_i24be")
    }

   fn prop_read_i32be(xs: i32) -> bool {
        let mut buf = Vec::new();
        buf.write_i32::<BigEndian>(xs).expect("write_i32");
        let mut reader = reader(&buf);
        xs == reader.read_i32be().expect("read_i32be")
    }

    fn prop_read_i48be(xs: (u16, u16, u16)) -> bool {
        let mut buf = Vec::new();
        let ys = (i64::from(xs.0) << 32) + (i64::from(xs.1) << 16) + i64::from(xs.2);
        buf.write_u16::<BigEndian>(xs.0).expect("write_u16");
        buf.write_u16::<BigEndian>(xs.1).expect("write_u16");
        buf.write_u16::<BigEndian>(xs.2).expect("write_u16");
        let mut reader = reader(&buf);
        ys == reader.read_i48be().expect("read_i48be")
    }

    fn prop_read_i64be(xs: i64) -> bool {
        let mut buf = Vec::new();
        buf.write_i64::<BigEndian>(xs).expect("write_i64");
        let mut reader = reader(&buf);
        xs == reader.read_i64be().expect("read_i64be")
    }

    fn prop_read_i128be(xs: (u64, u64)) -> bool {
        let mut buf = Vec::new();
        let ys = (i128::from(xs.0) << 64) + i128::from(xs.1);
        buf.write_u64::<BigEndian>(xs.0).expect("write_u64");
        buf.write_u64::<BigEndian>(xs.1).expect("write_u64");
        let mut reader = reader(&buf);
        ys == reader.read_i128be().expect("read_i128be")
    }

    fn prop_read_i16le(xs: i16) -> bool {
        let mut buf = Vec::new();
        buf.write_i16::<LittleEndian>(xs).expect("write_i16");
        let mut reader = reader(&buf);
        xs == reader.read_i16le().expect("read_i16le")
    }

    fn prop_read_i24le(xs: (u8, u8, u8)) -> bool {
        let mut buf = Vec::new();
        let ys = (i32::from(xs.0) << 16) + (i32::from(xs.1) << 8) + i32::from(xs.2);
        buf.write_i24::<LittleEndian>(ys).expect("write_i24");
        let mut reader = reader(&buf);
        ys == reader.read_i24le().expect("read_i24le")
    }

    fn prop_read_i32le(xs: i32) -> bool {
        let mut buf = Vec::new();
        buf.write_i32::<LittleEndian>(xs).expect("write_i32");
        let mut reader = reader(&buf);
        xs == reader.read_i32le().expect("read_i32le")
    }

    fn prop_read_i48le(xs: (u16, u16, u16)) -> bool {
        let mut buf = Vec::new();
        let ys = (i64::from(xs.0) << 32) + (i64::from(xs.1) << 16) + i64::from(xs.2);
        buf.write_u16::<LittleEndian>(xs.2).expect("write_u16");
        buf.write_u16::<LittleEndian>(xs.1).expect("write_u16");
        buf.write_u16::<LittleEndian>(xs.0).expect("write_u16");
        let mut reader = reader(&buf);
        ys == reader.read_i48le().expect("read_i48le")
    }

    fn prop_read_i64le(xs: i64) -> bool {
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(xs).expect("write_i64");
        let mut reader = reader(&buf);
        xs == reader.read_i64le().expect("read_i64le")
    }

    fn prop_read_i128le(xs: (u64, u64)) -> bool {
        let mut buf = Vec::new();
        let ys = (i128::from(xs.0) << 64) + i128::from(xs.1);
        buf.write_u64::<LittleEndian>(xs.1).expect("write_u64");
        buf.write_u64::<LittleEndian>(xs.0).expect("write_u64");
        let mut reader = reader(&buf);
        ys == reader.read_i128le().expect("read_i128le")
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
fn read_i8_specials() {
    let specials = vec![
        i8::min_value(),
        i8::min_value() + 1,
        -1,
        0,
        1,
        i8::max_value() - 1,
        i8::max_value(),
    ];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i8(s).expect("write_i8");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i8().expect("read_i8"));
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
fn read_i24be_specials() {
    let specials = vec![-8_388_608, -8_388_607, -1, 0, 1, 8_388_606, 8_388_607];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i24::<BigEndian>(s).expect("write_i24");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i24be().expect("read_i24be"));
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
fn read_i48be_specials() {
    let specials = vec![
        -140_737_488_355_328,
        -140_737_488_355_327,
        -1,
        0,
        1,
        140_737_488_355_326,
        140_737_488_355_327,
    ];
    for s in specials {
        let mut buf = Vec::new();
        let b1: u16 = ((s & 0xFFFF_0000_0000) >> 32) as u16;
        let b2: u16 = ((s & 0x0000_FFFF_0000) >> 16) as u16;
        let b3: u16 = (s & 0x0000_0000_FFFF) as u16;
        buf.write_u16::<BigEndian>(b1).expect("write_u16");
        buf.write_u16::<BigEndian>(b2).expect("write_u16");
        buf.write_u16::<BigEndian>(b3).expect("write_u16");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i48be().expect("read_i48be"));
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
fn read_i24le_specials() {
    let specials = vec![-8_388_608, -8_388_607, -1, 0, 1, 8_388_606, 8_388_607];
    for s in specials {
        let mut buf = Vec::new();
        buf.write_i24::<LittleEndian>(s).expect("write_i24");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i24le().expect("read_i24le"));
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
fn read_i48le_specials() {
    let specials = vec![
        -140_737_488_355_328,
        -140_737_488_355_327,
        -1,
        0,
        1,
        140_737_488_355_326,
        140_737_488_355_327,
    ];
    for s in specials {
        let mut buf = Vec::new();
        let b1: u16 = ((s & 0xFFFF_0000_0000) >> 32) as u16;
        let b2: u16 = ((s & 0x0000_FFFF_0000) >> 16) as u16;
        let b3: u16 = (s & 0x0000_0000_FFFF) as u16;
        buf.write_u16::<LittleEndian>(b3).expect("write_u16");
        buf.write_u16::<LittleEndian>(b2).expect("write_u16");
        buf.write_u16::<LittleEndian>(b1).expect("write_u16");
        let mut reader = reader(&buf);
        assert_eq!(s, reader.read_i48le().expect("read_i48le"));
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
#[cfg(feature = "use_std")]
fn read_utf16_with_odd_length() {
    let mut reader = reader(&[]);
    match reader.read_utf16(3) {
        Err(err) => assert_eq!(err, untrustended::Error::ParseError),
        _ => panic!("Test shouldn't reach here"),
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_ipv4addr() {
    use std::net::Ipv4Addr;
    use std::io::Write;
    let addrs: Vec<Ipv4Addr> = vec![
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
        assert_eq!(addr, reader.read_ipv4addr().expect("read_ipv4addr"));
    }
}

#[test]
#[cfg(feature = "use_std")]
fn read_ipv6addr() {
    use std::net::Ipv6Addr;
    use std::io::Write;
    let addrs: Vec<Ipv6Addr> = vec![
        "2001:DB8::".parse().expect("parse ipv6 addr"),
        "2001:DB8:ff00:00ff:f00f:0ff0:0000:ffff".parse().expect(
            "parse ipv6 addr"
        ),
        "2001:DB8:ffff:ffff:ffff:ffff:ffff:ffff".parse().expect(
            "parse ipv6 addr"
        ),
    ];
    for addr in addrs {
        let mut buf = Vec::new();
        buf.write_all(&addr.octets()).expect("write_all");
        let mut reader = reader(&buf);
        assert_eq!(addr, reader.read_ipv6addr().expect("read_ipv6addr"));
    }
}

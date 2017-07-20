extern crate byteorder;
#[macro_use]
extern crate quickcheck;

extern crate untrusted;
extern crate untrustended;

use untrusted::{Input, Reader};
use untrustended::ReaderExt;

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
}

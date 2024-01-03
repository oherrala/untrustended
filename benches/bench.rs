use std::net::{Ipv4Addr, Ipv6Addr};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use untrusted::{Input, Reader};
use untrustended::ReaderExt;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    let ipv4_header = generate_ipv4_header();
    let ipv6_header = generate_ipv6_header();

    c.bench_function("parse_ipv4_header", |b| {
        let input = Input::from(&ipv4_header);
        b.iter(|| black_box(input.read_all(untrustended::Error::EndOfInput, parse_ipv4_header)))
    });

    c.bench_function("parse_ipv6_header", |b| {
        let input = Input::from(&ipv6_header);
        b.iter(|| black_box(input.read_all(untrustended::Error::EndOfInput, parse_ipv6_header)))
    });
}

fn parse_ipv4_header(input: &mut Reader) -> Result<(Ipv4Addr, Ipv4Addr), untrustended::Error> {
    // RFC791 Section 3.1
    //
    //  0                   1                   2                   3
    //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |Version|  IHL  |Type of Service|          Total Length         |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |         Identification        |Flags|      Fragment Offset    |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |  Time to Live |    Protocol   |         Header Checksum       |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                       Source Address                          |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                    Destination Address                        |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                    Options                    |    Padding    |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

    let first_byte = input.read_u8()?;
    if (first_byte >> 4) != 4 {
        return Err(untrustended::Error::ParseError);
    }

    let _tos = input.read_u8()?;
    let _total_length = usize::from(input.read_u16be()?);
    let _identification = input.read_u16be()?;
    let _flags_and_offset = input.read_u16be()?;
    let _ttl = input.read_u8()?;
    let _protocol = input.read_u8()?;
    let _checksum = input.read_u16be()?;
    let src_ip = input.read_ipv4addr()?;
    let dst_ip = input.read_ipv4addr()?;

    Ok((src_ip, dst_ip))
}

fn generate_ipv4_header() -> Vec<u8> {
    let mut buf = Vec::with_capacity(40);
    buf.push((4 << 4) + 5); // Version + IHL
    buf.push(0); // Type of Service
    buf.extend_from_slice(&[0, 0]); // Total Length
    buf.extend_from_slice(&[0, 0]); // Identification
    buf.extend_from_slice(&[0, 0]); // Flags + Fragment Offset
    buf.push(128); // Time to Live
    buf.push(132); // Protocol: SCTP = 132
    buf.extend_from_slice(&[0, 0]); // Checksum
    buf.extend_from_slice(&[192, 0, 2, 42]); // Source Address
    buf.extend_from_slice(&[203, 0, 113, 13]); // Destination Address
    buf
}

fn parse_ipv6_header(input: &mut Reader) -> Result<(Ipv6Addr, Ipv6Addr), untrustended::Error> {
    // RFC2460 Section 3.
    //
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |Version| Traffic Class |           Flow Label                  |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |         Payload Length        |  Next Header  |   Hop Limit   |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                                                               |
    // +                                                               +
    // |                                                               |
    // +                         Source Address                        +
    // |                                                               |
    // +                                                               +
    // |                                                               |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                                                               |
    // +                                                               +
    // |                                                               |
    // +                      Destination Address                      +
    // |                                                               |
    // +                                                               +
    // |                                                               |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

    let first_quad = input.read_u32be()?;
    if (first_quad >> 28) != 6 {
        return Err(untrustended::Error::ParseError);
    }
    let _length = input.read_u16be()?;
    let _next_header = input.read_u8()?;
    let _hop_limit = input.read_u8()?;
    let src_ip = input.read_ipv6addr()?;
    let dst_ip = input.read_ipv6addr()?;

    Ok((src_ip, dst_ip))
}

fn generate_ipv6_header() -> Vec<u8> {
    let mut buf = Vec::with_capacity(40);
    buf.extend_from_slice(&[6 << 4, 0, 0, 0]); // Version + Traffic Class + Flow Label
    buf.extend_from_slice(&[0, 0]); // Payload Length
    buf.push(132); // Next Header: SCTP = 132
    buf.push(128); // Hop Limit
    buf.extend_from_slice(&[0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0x01]); // Source Address
    buf.extend_from_slice(&[0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0x02]); // Destination Address
    buf
}

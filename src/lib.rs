//! # Untrustended - Untrusted Extended.
//!
//! Untrustended is a compilation of primitives for parsing values from
//! untrusted input. It's building on top of
//! [untrusted](https://crates.io/crates/untrusted)'s
//! [`Reader::read_byte()`](https://briansmith.org/rustdoc/untrusted/struct.Reader.html#method.read_byte).
//!
//! Please, consult [untrusted](https://crates.io/crates/untrusted)'s
//! documentation about how to use that crate before attempting to use this one.
//!
//! To use the new methods provided by this crate:
//!
//! ```rust,no_run
//! use untrustended::ReaderExt;
//! ```
//!
//! then construct a `Reader` as usual and enjoy.
//!
//! Example:
//!
//! ```rust
//! extern crate untrusted;
//! extern crate untrustended;
//!
//! use untrusted::{Input, Reader};
//! use untrustended::{ReaderExt, Error};
//!
//! fn read_stuff(input: &mut Reader) -> Result<(u8, u16, u32), Error> {
//!     let one_byte = input.read_u8()?;
//!     let big_endian_u16 = input.read_u16be()?;
//!     let little_endian_u32 = input.read_u32le()?;
//!     Ok((one_byte, big_endian_u16, little_endian_u32))
//! }
//!
//! fn main() {
//!     let buf = vec![0, 1, 2, 3, 4, 5, 6];
//!     let input = Input::from(&buf);
//!
//!     input.read_all(Error::UnknownError, read_stuff).expect("read_all to succeed");
//! }
//! ```

// Copy lints from untrusted
// https://github.com/briansmith/untrusted/commit/01cde2d54e2f8fc234a8b5ea660fe510db2cf399

#![allow(
    missing_copy_implementations,
    missing_debug_implementations,
)]

// `#[derive(...)]` uses `#[allow(unused_qualifications)]` internally.
#![deny(
    unused_qualifications,
)]

#![forbid(
    anonymous_parameters,
    box_pointers,
    fat_ptr_transmutes,
    legacy_directory_ownership,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences,
    warnings,
)]

#![cfg_attr(not(feature = "use_std"), no_std)]

extern crate untrusted;
use untrusted::{Reader, Input, EndOfInput};

pub use error::Error;

#[cfg(feature = "use_std")]
use std::net::{Ipv4Addr, Ipv6Addr};

/// A trait extending [untrusted](https://crates.io/crates/untrusted)'s
/// [`Reader`](https://briansmith.org/rustdoc/untrusted/struct.Reader.html).
pub trait ReaderExt<'a> {
    /// Read one byte. This is the basic building block of every other read
    /// method provided.
    fn read_byte(&mut self) -> Result<u8, EndOfInput>;

    /// Skips num_bytes of the input, returning the skipped input as an Input.
    ///
    /// Returns Ok(i) where i is an Input if there are at least num_bytes of
    /// input remaining, and Err(EndOfInput) otherwise.
    fn skip_and_get_input(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>;

    /// Reads 8 bit unsigned integer.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader is at the end of the input.
    #[inline]
    fn read_u8(&mut self) -> Result<u8, Error> {
        self.read_byte().map_err(From::from)
    }

    /// Reads 16 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u16be(&mut self) -> Result<u16, Error> {
        let b1 = self.read_u8()? as u16;
        let b2 = self.read_u8()? as u16;
        Ok((b1 << 8) + b2)
    }

    /// Reads 24 bit unsigned integer in big endian.
    ///
    /// This method reads three bytes, but returns `u32` because Rust doesn't
    /// have 24 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u24be(&mut self) -> Result<u32, Error> {
        let b1 = self.read_u16be()? as u32;
        let b2 = self.read_u8()? as u32;
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u32be(&mut self) -> Result<u32, Error> {
        let b1 = self.read_u16be()? as u32;
        let b2 = self.read_u16be()? as u32;
        Ok((b1 << 16) + b2)
    }

    /// Reads 48 bit unsigned integer in big endian.
    ///
    /// This method reads six bytes, but returns `u64` because Rust doesn't have
    /// 48 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u48be(&mut self) -> Result<u64, Error> {
        let b1 = self.read_u24be()? as u64;
        let b2 = self.read_u24be()? as u64;
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u64be(&mut self) -> Result<u64, Error> {
        let b1 = self.read_u32be()? as u64;
        let b2 = self.read_u32be()? as u64;
        Ok((b1 << 32) + b2)
    }

    /// Reads 16 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u16le(&mut self) -> Result<u16, Error> {
        let b2 = self.read_u8()? as u16;
        let b1 = self.read_u8()? as u16;
        Ok((b1 << 8) + b2)
    }

    /// Reads 24 bit unsigned integer in little endian.
    ///
    /// This method reads three bytes, but returns `u32` because Rust doesn't
    /// have 24 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u24le(&mut self) -> Result<u32, Error> {
        let b2 = self.read_u8()? as u32;
        let b1 = self.read_u16le()? as u32;
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u32le(&mut self) -> Result<u32, Error> {
        let b2 = self.read_u16le()? as u32;
        let b1 = self.read_u16le()? as u32;
        Ok((b1 << 16) + b2)
    }

    /// Reads 48 bit unsigned integer in little endian.
    ///
    /// This method reads six bytes, but returns `u64` because Rust doesn't have
    /// 48 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u48le(&mut self) -> Result<u64, Error> {
        let b2 = self.read_u24le()? as u64;
        let b1 = self.read_u24le()? as u64;
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u64le(&mut self) -> Result<u64, Error> {
        let b2 = self.read_u32le()? as u64;
        let b1 = self.read_u32le()? as u64;
        Ok((b1 << 32) + b2)
    }

    /// Reads 8 bit signed integer.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader is at the end of the input.
    #[inline]
    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads 16 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i16be(&mut self) -> Result<i16, Error> {
        let b1 = self.read_u8()? as i16;
        let b2 = self.read_u8()? as i16;
        Ok((b1 << 8) + b2)
    }

    /// Reads 24 bit signed integer in big endian.
    ///
    /// This method reads three bytes, but returns `i32` because Rust doesn't
    /// have 24 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i24be(&mut self) -> Result<i32, Error> {
        let b1 = self.read_i16be()? as i32;
        let b2 = self.read_u8()? as i32;
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i32be(&mut self) -> Result<i32, Error> {
        let b1 = self.read_u16be()? as i32;
        let b2 = self.read_u16be()? as i32;
        Ok((b1 << 16) + b2)
    }

    /// Reads 48 bit signed integer in big endian.
    ///
    /// This method reads six bytes, but returns `i64` because Rust doesn't have
    /// 48 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i48be(&mut self) -> Result<i64, Error> {
        let b1 = self.read_i24be()? as i64;
        let b2 = self.read_u24be()? as i64;
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i64be(&mut self) -> Result<i64, Error> {
        let b1 = self.read_u32be()? as i64;
        let b2 = self.read_u32be()? as i64;
        Ok((b1 << 32) + b2)
    }

    /// Reads 16 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i16le(&mut self) -> Result<i16, Error> {
        let b2 = self.read_u8()? as i16;
        let b1 = self.read_u8()? as i16;
        Ok((b1 << 8) + b2)
    }

    /// Reads 24 bit signed integer in little endian.
    ///
    /// This method reads three bytes, but returns `i32` because Rust doesn't
    /// have 24 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i24le(&mut self) -> Result<i32, Error> {
        let b2 = self.read_u8()? as i32;
        let b1 = self.read_i16le()? as i32;
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i32le(&mut self) -> Result<i32, Error> {
        let b2 = self.read_u16le()? as i32;
        let b1 = self.read_u16le()? as i32;
        Ok((b1 << 16) + b2)
    }

    /// Reads 48 bit signed integer in little endian.
    ///
    /// This method reads six bytes, but returns `i64` because Rust doesn't have
    /// 48 bit integer type.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i48le(&mut self) -> Result<i64, Error> {
        let b2 = self.read_u24le()? as i64;
        let b1 = self.read_i24le()? as i64;
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i64le(&mut self) -> Result<i64, Error> {
        let b2 = self.read_u32le()? as i64;
        let b1 = self.read_u32le()? as i64;
        Ok((b1 << 32) + b2)
    }

    /// Reads given amount of bytes.
    ///
    /// Access the given amount of bytes as a slice so it can be processed by
    /// functions that are not written using the untrusted's Input/Reader
    /// framework.
    ///
    /// Returns Ok(v) where v is a `&[u8]` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading.
    #[inline]
    fn read_bytes_less_safe(&mut self, num_bytes: usize) -> Result<&'a [u8], Error> {
        Ok(self.skip_and_get_input(num_bytes).map(
            |v| v.as_slice_less_safe(),
        )?)
    }

    /// Reads bytes as UTF-8 String.
    ///
    /// Length required is the amount of bytes to read, not the amount of UTF-8
    /// characters.
    ///
    /// Read bytes are validated to be valid UTF-8 by
    /// [`str::from_utf8`](https://doc.rust-lang.org/std/str/fn.from_utf8.html)
    /// method.
    ///
    /// Returns Ok(v) where v is a `&str` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading, or Err(Error::ParseError) if UTF-8 parsing failed.
    #[inline]
    #[cfg(feature = "use_std")]
    fn read_utf8(&mut self, num_bytes: usize) -> Result<&'a str, Error> {
        let buf = self.read_bytes_less_safe(num_bytes)?;
        std::str::from_utf8(buf).map_err(From::from)
    }

    /// Reads bytes as UTF-16 String.
    ///
    /// Length is the amount of bytes to read, not the amount of UTF-16
    /// characters. Length should be even number and Err(Error::ParseError) is
    /// returned if it's odd.
    ///
    /// Read bytes are validated to be valid UTF-16 by
    /// [`String::from_utf16`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf16)
    /// method.
    ///
    /// Returns Ok(v) where v is a `String` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading, or Err(Error::ParseError) if UTF-8 parsing failed.
    #[inline]
    #[cfg(feature = "use_std")]
    fn read_utf16(&mut self, num_bytes: usize) -> Result<String, Error> {
        if (num_bytes % 2) != 0 {
            return Err(Error::ParseError);
        }
        let len16 = num_bytes / 2;
        let mut buf: Vec<u16> = Vec::with_capacity(len16);
        for _ in 0..len16 {
            let b = self.read_u16be()?;
            buf.push(b);
        }
        String::from_utf16(&buf).map_err(From::from)
    }

    /// Reads IPv4 address in big endian format.
    ///
    /// Returns Ok(v) where v is a `Ipv4Addr`, or Err(Error::EndOfInput) if the
    /// Reader encountered an end of the input while reading, or
    /// Err(Error::ParseError) if parsing of address failed.
    #[inline]
    #[cfg(feature = "use_std")]
    fn read_ipv4addr(&mut self) -> Result<Ipv4Addr, Error> {
        let bytes = self.read_u32be()?;
        Ok(Ipv4Addr::from(bytes))
    }

    /// Reads IPv6 address in big endian format.
    ///
    /// Returns Ok(v) where v is a `Ipv6Addr`, or Err(Error::EndOfInput) if the
    /// Reader encountered an end of the input while reading, or
    /// Err(Error::ParseError) if parsing of address failed.
    #[inline]
    #[cfg(feature = "use_std")]
    fn read_ipv6addr(&mut self) -> Result<Ipv6Addr, Error> {
        let mut b = [0u16; 8];
        for i in &mut b {
            *i = self.read_u16be()?;
        }
        let ip = Ipv6Addr::new(b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]);
        Ok(ip)
    }
}

impl<'a> ReaderExt<'a> for Reader<'a> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, EndOfInput> {
        self.read_byte()
    }

    #[inline]
    fn skip_and_get_input(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput> {
        self.skip_and_get_input(num_bytes)
    }
}

mod error {
    #[cfg(feature = "use_std")]
    use std::fmt;
    #[cfg(feature = "use_std")]
    use std::str::Utf8Error;
    #[cfg(feature = "use_std")]
    use std::string::FromUtf16Error;
    use untrusted::EndOfInput;

    /// Possible errors raised by `ReaderExt`.
    #[derive(Debug, PartialEq)]
    pub enum Error {
        /// The error type used to indicate the end of the input was reached
        /// before the operation could be completed.
        EndOfInput,
        /// The error type used to indicate when a failed parsing while trying
        /// to convert bytes into a more specific type.
        ParseError,
        /// Unknown error occured.
        UnknownError,
    }

    #[cfg(feature = "use_std")]
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "reading failed with {:?}", self)
        }
    }

    impl From<EndOfInput> for Error {
        fn from(_: EndOfInput) -> Self {
            Error::EndOfInput
        }
    }

    #[cfg(feature = "use_std")]
    impl From<Utf8Error> for Error {
        fn from(_: Utf8Error) -> Self {
            Error::ParseError
        }
    }

    #[cfg(feature = "use_std")]
    impl From<FromUtf16Error> for Error {
        fn from(_: FromUtf16Error) -> Self {
            Error::ParseError
        }
    }
}

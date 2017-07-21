//! # Untrustended - Untrusted Extended.
//!
//! Untrustended is a compilation of primitives for parsing values from
//! untrusted input. It's building on top of
//! [untrusted](https://crates.io/crates/untrusted)'s
//! [`Reader::read_byte()`](../untrusted/struct.Reader.html#method.read_byte).
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

#![deny(warnings, missing_docs)]

extern crate untrusted;
use untrusted::Reader;

pub use error::Error;

/// A trait extending [untrusted](https://crates.io/crates/untrusted)'s
/// [`Reader`](../untrusted/struct.Reader.html).
pub trait ReaderExt {
    /// Read one byte. This is the basic building block of every other read
    /// method provided.
    fn read_byte(&mut self) -> Result<u8, untrusted::EndOfInput>;

    /// Reads 8 bit unsigned integer. Same as `read_byte`.
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
    /// Returns Ok(v) where v is a `Vec<u8>` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading.
    #[inline]
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(length);
        for _ in 0..length {
            let b = self.read_byte()?;
            buf.push(b);
        }
        Ok(buf)
    }

    /// Reads bytes as UTF-8 String.
    ///
    /// Length is the amount of bytes to read, not the amount of UTF-8
    /// characters.
    ///
    /// Read bytes are validated to be valid UTF-8 by
    /// [String::from_utf8](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8)
    /// method.
    ///
    /// Returns Ok(v) where v is a `String` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading, or Err(Error::UnicodeError) if UTF-8 parsing failed.
    #[inline]
    fn read_utf8(&mut self, length: usize) -> Result<String, Error> {
        let buf = self.read_bytes(length)?;
        String::from_utf8(buf).map_err(From::from)
    }

    /// Reads bytes as UTF-16 String.
    ///
    /// Length is the amount of bytes to read, not the amount of UTF-16
    /// characters.
    ///
    /// Read bytes are validated to be valid UTF-16 by
    /// [String::from_utf16](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf16)
    /// method.
    ///
    /// Returns Ok(v) where v is a `String` of bytes read, or
    /// Err(Error::EndOfInput) if the Reader encountered an end of the input
    /// while reading, or Err(Error::UnicodeError) if UTF-8 parsing failed.
    #[inline]
    fn read_utf16(&mut self, length: usize) -> Result<String, Error> {
        let len16 = length / 2;
        let mut buf: Vec<u16> = Vec::with_capacity(len16);
        for _ in 0..len16 {
            let b = self.read_u16be()?;
            buf.push(b);
        }
        String::from_utf16(&buf).map_err(From::from)
    }
}

impl<'a> ReaderExt for Reader<'a> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, untrusted::EndOfInput> {
        self.read_byte()
    }
}

mod error {
    use std::fmt;
    use std::string::{FromUtf8Error, FromUtf16Error};
    use untrusted::EndOfInput;

    /// Possible errors raised by `ReaderExt`.
    #[derive(Debug)]
    pub enum Error {
        /// The error type used to indicate the end of the input was reached
        /// before the operation could be completed.
        EndOfInput,
        /// The error type used to indicate when a failed parsing while trying
        /// to convert bytes into a `String`.
        UnicodeError,
        /// Unknown error occured.
        UnknownError,
    }

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

    impl From<FromUtf8Error> for Error {
        fn from(_: FromUtf8Error) -> Self {
            Error::UnicodeError
        }
    }

    impl From<FromUtf16Error> for Error {
        fn from(_: FromUtf16Error) -> Self {
            Error::UnicodeError
        }
    }
}

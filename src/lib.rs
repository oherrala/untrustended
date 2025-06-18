//! # Untrustended - Untrusted Extended.
//!
//! Untrustended is a compilation of primitives for parsing values from
//! untrusted input. It's building on top of
//! [untrusted](https://crates.io/crates/untrusted)'s
//! [`Reader::read_byte()`](https://briansmith.org/rustdoc/untrusted/struct.Reader.html#method.read_byte)
//! and [`Reader::read_bytes()`](https://briansmith.org/rustdoc/untrusted/struct.Reader.html#method.read_bytes).
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
//! use untrusted::{Input, Reader};
//! use untrustended::{ReaderExt, Error};
//!
//! fn read_stuff(input: &mut Reader<'_>) -> Result<(u8, u16, u32), Error> {
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

#![cfg_attr(not(feature = "use_std"), no_std)]

use untrusted::{EndOfInput, Input, Reader};

pub use crate::error::Error;

#[cfg(feature = "use_std")]
use std::net::{Ipv4Addr, Ipv6Addr};

/// A trait extending [untrusted](https://crates.io/crates/untrusted)'s
/// [`Reader`](https://briansmith.org/rustdoc/untrusted/struct.Reader.html).
pub trait ReaderExt<'a> {
    /// Read one byte.
    fn read_byte(&mut self) -> Result<u8, EndOfInput>;

    /// Skips num_bytes of the input, returning the skipped input as an Input.
    ///
    /// Returns Ok(i) where i is an Input if there are at least num_bytes of
    /// input remaining, and Err(EndOfInput) otherwise.
    fn read_bytes(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>;

    /// Read as many bytes as needed to instantiate a type in Big Endian byte
    /// order.
    fn read_be<T: FromReader>(&mut self) -> Result<T, Error>;

    /// Read as many bytes as needed to instantiate a type in Little Endian byte
    /// order.
    fn read_le<T: FromReader>(&mut self) -> Result<T, Error>;

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
        self.read_be()
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
        let b1 = u32::from(self.read_u16be()?);
        let b2 = u32::from(self.read_u8()?);
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u32be(&mut self) -> Result<u32, Error> {
        self.read_be()
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
        let b1 = u64::from(self.read_u24be()?);
        let b2 = u64::from(self.read_u24be()?);
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u64be(&mut self) -> Result<u64, Error> {
        self.read_be()
    }

    /// Reads 128 bit unsigned integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u128be(&mut self) -> Result<u128, Error> {
        self.read_be()
    }

    /// Reads 16 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u16le(&mut self) -> Result<u16, Error> {
        self.read_le()
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
        let b2 = u32::from(self.read_u8()?);
        let b1 = u32::from(self.read_u16le()?);
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u32le(&mut self) -> Result<u32, Error> {
        self.read_le()
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
        let b2 = u64::from(self.read_u24le()?);
        let b1 = u64::from(self.read_u24le()?);
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u64le(&mut self) -> Result<u64, Error> {
        self.read_le()
    }

    /// Reads 128 bit unsigned integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_u128le(&mut self) -> Result<u128, Error> {
        self.read_le()
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
        self.read_be()
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
        let b1 = i32::from(self.read_i16be()?);
        let b2 = i32::from(self.read_u8()?);
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i32be(&mut self) -> Result<i32, Error> {
        self.read_be()
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
        let b1 = i64::from(self.read_i24be()?);
        let b2 = i64::from(self.read_u24be()?);
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i64be(&mut self) -> Result<i64, Error> {
        self.read_be()
    }

    /// Reads 128 bit signed integer in big endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i128be(&mut self) -> Result<i128, Error> {
        self.read_be()
    }

    /// Reads 16 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i16le(&mut self) -> Result<i16, Error> {
        self.read_le()
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
        let b2 = i32::from(self.read_u8()?);
        let b1 = i32::from(self.read_i16le()?);
        Ok((b1 << 8) + b2)
    }

    /// Reads 32 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i32le(&mut self) -> Result<i32, Error> {
        self.read_le()
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
        let b2 = i64::from(self.read_u24le()?);
        let b1 = i64::from(self.read_i24le()?);
        Ok((b1 << 24) + b2)
    }

    /// Reads 64 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i64le(&mut self) -> Result<i64, Error> {
        self.read_le()
    }

    /// Reads 128 bit signed integer in little endian.
    ///
    /// Returns Ok(v) where v is the value read, or Err(Error::EndOfInput) if
    /// the Reader encountered an end of the input while reading.
    #[inline]
    fn read_i128le(&mut self) -> Result<i128, Error> {
        self.read_le()
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
        Ok(self.read_bytes(num_bytes).map(|v| v.as_slice_less_safe())?)
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
        self.read_u32be().map(Ipv4Addr::from)
    }

    /// Reads IPv6 address in big endian format.
    ///
    /// Returns Ok(v) where v is a `Ipv6Addr`, or Err(Error::EndOfInput) if the
    /// Reader encountered an end of the input while reading, or
    /// Err(Error::ParseError) if parsing of address failed.
    #[inline]
    #[cfg(feature = "use_std")]
    fn read_ipv6addr(&mut self) -> Result<Ipv6Addr, Error> {
        self.read_u128be().map(Ipv6Addr::from)
    }
}

impl<'a> ReaderExt<'a> for Reader<'a> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, EndOfInput> {
        self.read_byte()
    }

    #[inline]
    fn read_bytes(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput> {
        self.read_bytes(num_bytes)
    }

    #[inline]
    fn read_be<T: FromReader>(&mut self) -> Result<T, Error> {
        FromReader::read_be(self)
    }

    #[inline]
    fn read_le<T: FromReader>(&mut self) -> Result<T, Error> {
        FromReader::read_le(self)
    }
}

/// A trait to abstract the idea of creating a new instance of a type from
/// reading bytes out from `Reader`.
pub trait FromReader: Sized {
    /// Read as many bytes as needed to instantiate a type in Big Endian byte
    /// order.
    fn read_be(_: &mut Reader<'_>) -> Result<Self, Error>;

    /// Read as many bytes as needed to instantiate a type in Little Endian byte
    /// order.
    fn read_le(_: &mut Reader<'_>) -> Result<Self, Error>;
}

macro_rules! read_unsigned {
    ($type:ty) => {
        #[inline]
        fn read_be(reader: &mut Reader<'_>) -> Result<Self, Error> {
            const LEN: usize = core::mem::size_of::<$type>();
            let mut arr = [0u8; LEN];
            let slice = reader.read_bytes(LEN)?.as_slice_less_safe();
            arr.copy_from_slice(slice);
            Ok(<$type>::from_be_bytes(arr))
        }

        #[inline]
        fn read_le(reader: &mut Reader<'_>) -> Result<Self, Error> {
            const LEN: usize = core::mem::size_of::<$type>();
            let mut arr = [0u8; LEN];
            let slice = reader.read_bytes(LEN)?.as_slice_less_safe();
            arr.copy_from_slice(slice);
            Ok(<$type>::from_le_bytes(arr))
        }
    };
}

macro_rules! read_signed {
    ($type:ty) => {
        #[inline]
        fn read_be(reader: &mut Reader<'_>) -> Result<Self, Error> {
            let r = reader.read_be::<$type>()?;
            Ok(r as Self)
        }

        #[inline]
        fn read_le(reader: &mut Reader<'_>) -> Result<Self, Error> {
            let r = reader.read_le::<$type>()?;
            Ok(r as Self)
        }
    };
}

impl FromReader for u8 {
    #[inline]
    fn read_be(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_byte().map_err(From::from)
    }

    #[inline]
    fn read_le(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_byte().map_err(From::from)
    }
}

impl FromReader for u16 {
    read_unsigned!(u16);
}
impl FromReader for u32 {
    read_unsigned!(u32);
}
impl FromReader for u64 {
    read_unsigned!(u64);
}
impl FromReader for u128 {
    read_unsigned!(u128);
}

impl FromReader for i8 {
    read_signed!(u8);
}
impl FromReader for i16 {
    read_signed!(u16);
}
impl FromReader for i32 {
    read_signed!(u32);
}
impl FromReader for i64 {
    read_signed!(u64);
}
impl FromReader for i128 {
    read_signed!(u128);
}

#[cfg(feature = "use_std")]
impl FromReader for Ipv4Addr {
    fn read_be(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_u32be().map(Ipv4Addr::from)
    }

    fn read_le(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_u32le().map(Ipv4Addr::from)
    }
}

#[cfg(feature = "use_std")]
impl FromReader for Ipv6Addr {
    fn read_be(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_u128be().map(Ipv6Addr::from)
    }

    fn read_le(reader: &mut Reader<'_>) -> Result<Self, Error> {
        reader.read_u128le().map(Ipv6Addr::from)
    }
}

/// Data structure that can be constructed by reading from [untrusted::Reader]
///
/// # Example
/// Read Type-Length-Value encoded data into `Data` struct.
///
/// ```
/// use untrustended::{Readable, ReaderExt, Error};
/// use untrusted::{Input, Reader};
///
/// struct Data {
///     t: u16,
///     val: Vec<u8>,
/// }
/// impl Readable for Data {
///     type Output = Data;
///     fn read(input: &mut Reader<'_>) -> Result<Self::Output, Error> {
///         let t = input.read_u16le()?;
///         let len = input.read_u16le()?;
///         let val = input.read_bytes_less_safe(usize::from(len))?.to_owned();
///         Ok(Data { t, val })
///     }
/// }
/// let input = [0x01u8, 0x00, 0x02, 0x00, 0xaa, 0xbb];
/// let data = Input::from(&input)
///     .read_all(Error::ParseError, Data::read)
///     .expect("could not parse Data");
/// assert_eq!(data.t, 0x01u16);
/// assert_eq!(data.val, input[4..]);
/// ```
pub trait Readable {
    /// Type this readable can produce
    type Output;
    /// Parses data from `input` and produces [Self::Output] instance from
    /// data read.
    fn read(input: &mut Reader<'_>) -> Result<Self::Output, Error>;
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
        /// The error type used to indicate when parsing failed while trying
        /// to convert bytes into a more specific type.
        ParseError,
        /// The error type indicating that while data parsed was syntactically
        /// correct, the value parsed vas invalid in this context.
        InvalidValue,
        /// Unknown error occured.
        UnknownError,
    }

    #[cfg(feature = "use_std")]
    impl std::error::Error for Error {}

    #[cfg(feature = "use_std")]
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Error::EndOfInput => f.write_str("end of input was reached unexpectedly"),
                Error::ParseError => f.write_str("failed to parse data into a more specific type"),
                Error::InvalidValue => f.write_str("parsed data contained invalid value"),
                Error::UnknownError => f.write_str("reading failed with an unknown error"),
            }
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

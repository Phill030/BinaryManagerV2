use std::{borrow::Borrow, io::Read, io::Write};

mod error;
mod memory;

pub use error::BinaryError;
pub use memory::memory::MemoryStream;

/// Result type for binary errors.
pub type Result<T> = std::result::Result<T, BinaryError>;

macro_rules! write_data {
    ($endian:expr, $value:expr, $stream:expr) => {
        let data = match $endian {
            Endian::Little => $value.to_le_bytes(),
            Endian::Big => $value.to_be_bytes(),
        };
        return Ok($stream.write(&data)?);
    };
}

macro_rules! read_data {
    ($endian:expr, $value:expr, $kind:ty) => {
        let data = match $endian {
            Endian::Little => <$kind>::from_le_bytes($value),
            Endian::Big => <$kind>::from_be_bytes($value),
        };
        return Ok(data);
    };
}

/// Variants to describe endianness.
#[derive(PartialEq)]
pub enum Endian {
    /// Big endian.
    Big,
    /// Little endian.
    Little,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Little
    }
}

//--    SeekStream      --\\
/// Trait for streams that can seek.
pub trait SeekStream {
    /// Seek to a position.
    fn seek(&mut self, to: usize) -> Result<usize>;
    /// Get the current position.
    fn tell(&mut self) -> Result<usize>;
    /// Get the length of the stream.
    fn len(&self) -> Result<usize>;
}

//--        BinaryReader      --\\
pub struct BinaryReader<'a> {
    pub stream: MemoryStream<'a>,
    endian: Endian,
}

impl<'a> BinaryReader<'a> {
    /// Createa a new BinaryWriter with a predefined MemoryStream
    pub fn new_stream<'b>(stream: MemoryStream<'a>, endian: Endian) -> Self {
        Self { endian, stream }
    }

    /// Createa a new BinaryWriter with a predefined buffer
    pub fn new_vec(buffer: &'a mut Vec<u8>, endian: Endian) -> Self {
        Self {
            endian,
            stream: MemoryStream::new_vec(buffer),
        }
    }

    /// Read a length-prefixed `String` from the stream.
    pub fn read_string(&mut self) -> Result<String> {
        let chars = if cfg!(feature = "wasm32") {
            let str_len = self.read_u32()?;
            let mut chars: Vec<u8> = vec![0; str_len as usize];
            self.stream.read(&mut chars)?;
            chars
        } else {
            let str_len = self.read_usize()?;
            let mut chars: Vec<u8> = vec![0; str_len];
            self.stream.read(&mut chars)?;
            chars
        };
        Ok(String::from_utf8(chars)?)
    }

    /// Swap endianness to allow for reversing the reading mid stream
    pub fn swap_endianness(&mut self) {
        if self.endian == Endian::Big {
            self.endian = Endian::Little;
        } else {
            self.endian = Endian::Big;
        }
    }

    /// Read a character from the stream.
    pub fn read_char(&mut self) -> Result<char> {
        Ok(std::char::from_u32(self.read_u32()?).ok_or_else(|| BinaryError::InvalidChar)?)
    }

    /// Read a `bool` from the stream.
    pub fn read_bool(&mut self) -> Result<bool> {
        let value = self.read_u8()?;
        Ok(value > 0)
    }

    /// Read a `f32` from the stream.
    pub fn read_f32(&mut self) -> Result<f32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, f32);
    }

    /// Read a `f64` from the stream.
    pub fn read_f64(&mut self) -> Result<f64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, f64);
    }

    /// Read an `isize` from the stream.
    #[cfg(target_arch = "wasm32")]
    pub fn read_isize(&mut self) -> Result<isize> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, isize);
    }

    /// Read an `isize` from the stream.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_isize(&mut self) -> Result<isize> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, isize);
    }

    /// Read a `usize` from the stream.
    #[cfg(target_arch = "wasm32")]
    pub fn read_usize(&mut self) -> Result<usize> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, usize);
    }

    /// Read a `usize` from the stream.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_usize(&mut self) -> Result<usize> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, usize);
    }

    /// Read a `u64` from the stream.
    pub fn read_u64(&mut self) -> Result<u64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, u64);
    }

    /// Read an `i64` from the stream.
    pub fn read_i64(&mut self) -> Result<i64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, i64);
    }

    /// Read a `u32` from the stream.
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, u32);
    }

    /// Read an `i32` from the stream.
    pub fn read_i32(&mut self) -> Result<i32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, i32);
    }

    /// Read a `u16` from the stream.
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, u16);
    }

    /// Read an `i16` from the stream.
    pub fn read_i16(&mut self) -> Result<i16> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, i16);
    }

    /// Read a `u8` from the stream.
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, u8);
    }

    /// Read an `i8` from the stream.
    pub fn read_i8(&mut self) -> Result<i8> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        read_data!(self.endian, buffer, i8);
    }

    /// Read bytes from the stream into a buffer.
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }

    /// same as `read_bytes` but at a certain offset
    pub fn read_bytes_at(&mut self, length: usize, position: usize) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length];
        self.stream.seek(position)?;
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads a string by reading the length before
    pub fn read_big_string(&mut self) -> Result<String> {
        let len = self.read_i32()? as usize;
        let str = self.read_bytes(len)?;
        Ok(String::from_utf8(str)?)
    }
}

//

//--        BinaryWriter      --\\
pub struct BinaryWriter<'a> {
    pub stream: MemoryStream<'a>,
    endian: Endian,
}

impl<'a> BinaryWriter<'a> {
    /// Createa a new BinaryWriter with predefined data
    pub fn new_stream<'b>(stream: MemoryStream<'b>, endian: Endian) -> Self
    where
        'b: 'a,
    {
        Self { endian, stream }
    }

    pub fn new_vec(stream: &'a mut Vec<u8>, endian: Endian) -> Self {
        Self {
            endian,
            stream: MemoryStream::new_vec(stream),
        }
    }

    /// Write a length-prefixed `String` to the stream.
    ///
    /// The length of the `String` is written as a `usize`
    /// unless the `wasm32` feature is enabled
    /// in which case the length is a `u32`.
    pub fn write_string<S: AsRef<str>>(&mut self, value: S) -> Result<usize> {
        let bytes = value.as_ref().as_bytes();
        if cfg!(feature = "wasm32") {
            self.write_u32(bytes.len() as u32)?;
        } else {
            self.write_usize(bytes.len())?;
        }
        Ok(self.stream.write(&bytes.to_vec())?)
    }

    /// Write a character to the stream.
    pub fn write_char<V: Borrow<char>>(&mut self, v: V) -> Result<usize> {
        self.write_u32(*v.borrow() as u32)
    }

    /// Write a `bool` to the stream.
    pub fn write_bool<V: Borrow<bool>>(&mut self, value: V) -> Result<usize> {
        let written = self.write_u8(if *value.borrow() { 1 } else { 0 })?;
        Ok(written)
    }

    /// Write a `f32` to the stream.
    pub fn write_f32<V: Borrow<f32>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `f64` to the stream.
    pub fn write_f64<V: Borrow<f64>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write an `isize` to the stream.
    pub fn write_isize<V: Borrow<isize>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `usize` to the stream.
    pub fn write_usize<V: Borrow<usize>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `u64` to the stream.
    pub fn write_u64<V: Borrow<u64>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write an `i64` to the stream.
    pub fn write_i64<V: Borrow<i64>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `u32` to the stream.
    pub fn write_u32<V: Borrow<u32>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write an `i32` to the stream.
    pub fn write_i32<V: Borrow<i32>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `u16` to the stream.
    pub fn write_u16<V: Borrow<u16>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write an `i16` to the stream.
    pub fn write_i16<V: Borrow<i16>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a `u8` to the stream.
    pub fn write_u8<V: Borrow<u8>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write an `i8` to the stream.
    pub fn write_i8<V: Borrow<i8>>(&mut self, value: V) -> Result<usize> {
        write_data!(self.endian, value.borrow(), self.stream);
    }

    /// Write a byte buffer to the stream.
    pub fn write_bytes<B: AsRef<[u8]>>(&mut self, data: B) -> Result<usize> {
        Ok(self.stream.write(data.as_ref())?)
    }

    /// Writes bytes at a certain position. Does not go back to it's original position!
    pub fn write_bytes_at<B: AsRef<[u8]>>(&mut self, data: B, position: usize) -> Result<usize> {
        self.stream.seek(position)?;
        Ok(self.stream.write(data.as_ref())?)
    }

    /// Writes a bigstring with an optional position
    pub fn write_big_string<B: AsRef<[u8]>>(
        &mut self,
        data: B,
        position: Option<usize>,
    ) -> Result<usize> {
        if let Some(pos) = position {
            self.stream.seek(pos)?;
        }
        Ok(self.stream.write(data.as_ref())?)
    }

    /// Write a byte buffer to the stream.
    pub fn write_bytes_with_value(&mut self, count: usize, fill_value: u8) -> Result<usize> {
        let mut buff = Vec::with_capacity(count) as Vec<u8>;
        buff.resize(count, fill_value);
        Ok(self.write_bytes(buff)?)
    }

    /// Swap endianness to allow for reversing the writing mid stream
    pub fn swap_endianness(&mut self) {
        if self.endian == Endian::Big {
            self.endian = Endian::Little;
        } else {
            self.endian = Endian::Big;
        }
    }
}

use std::io::{Error, ErrorKind, Read, Write};

use crate::{BinaryError, Result, SeekStream};

pub struct MemoryStream<'a> {
    pub buffer: &'a mut [u8],
    pub position: usize,
}

impl<'a> MemoryStream<'a> {
    pub fn new_vec(buffer: &'a mut Vec<u8>) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}

/// This implements the `SeekStream` trait from main.rs into `MemoryStream`
impl<'a> SeekStream for MemoryStream<'a> {
    /// TUpdates the position of a mutable reference to a struct and returns the new
    /// position.
    ///
    /// Arguments:
    ///
    /// * `to`: The `to` parameter is the position to which the seek operation should move the current
    /// position. It is of type `usize`
    ///
    /// Returns:
    ///
    /// The `seek` function is returning a `Result` enum with a value of `Ok(self.position)`.
    fn seek(&mut self, to: usize) -> Result<usize> {
        self.position = to;
        Ok(self.position)
    }
    /// Returns the current position of a mutable reference.
    ///
    /// Returns:
    ///
    /// The `tell` function is returning a `Result` enum with a value of `usize`.
    fn tell(&mut self) -> Result<usize> {
        Ok(self.position)
    }
    /// Returns the length of the buffer as a `Result` containing a `usize`.
    ///
    /// Returns:
    ///
    /// The `len` function is returning a `Result` type with a value of `usize`.
    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
}

impl<'a> Read for MemoryStream<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.position + buffer.len() > self.buffer.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                BinaryError::ReadPastEof,
            ));
        }

        let mut idx = 0;
        for i in self.position..self.position + buffer.len() {
            buffer[idx] = self.buffer[i];
            idx += 1;
        }

        self.position += buffer.len();

        Ok(buffer.len())
    }
}

impl<'a> Write for MemoryStream<'a> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.buffer.clone_from_slice(bytes);
        self.position += bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Into<Vec<u8>> for MemoryStream<'_> {
    fn into(self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}

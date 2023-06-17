use std::io::{Error, ErrorKind, Read, Write};

use crate::{BinaryError, Result, SeekStream};

#[derive(Clone)]
pub struct MemoryStream {
    buffer: Vec<u8>,
    position: usize,
}

impl MemoryStream {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    pub fn new_vec(buffer: Vec<u8>) -> Self {
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
/// it contains following functions:
/// fn seek(&mut self, to: usize) -> Result<usize>;
/// fn tell(&mut self) -> Result<usize>;
/// fn len(&self) -> Result<usize>;
impl SeekStream for MemoryStream {
    fn seek(&mut self, to: usize) -> Result<usize> {
        self.position = to;
        Ok(self.position)
    }
    fn tell(&mut self) -> Result<usize> {
        Ok(self.position)
    }
    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
}

impl Read for MemoryStream {
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

impl Write for MemoryStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(bytes);
        self.position += bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl From<Vec<u8>> for MemoryStream {
    fn from(buffer: Vec<u8>) -> Self {
        MemoryStream {
            buffer,
            position: 0,
        }
    }
}

impl Into<Vec<u8>> for MemoryStream {
    fn into(self) -> Vec<u8> {
        self.buffer
    }
}

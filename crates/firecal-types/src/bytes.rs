//! Reading and writing little-endian numbers on byte slices, without indexing and without panics.

use crate::Error;

/// Reads a `u16` at byte offset `at`, or `None` when the slice is too short.
pub(crate) fn read_u16(bytes: &[u8], at: usize) -> Option<u16> {
    let raw = bytes.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes(<[u8; 2]>::try_from(raw).ok()?))
}

/// Reads a `u32` at byte offset `at`, or `None` when the slice is too short.
pub(crate) fn read_u32(bytes: &[u8], at: usize) -> Option<u32> {
    let raw = bytes.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes(<[u8; 4]>::try_from(raw).ok()?))
}

/// Reads an `f32` at byte offset `at`, or `None` when the slice is too short.
pub(crate) fn read_f32(bytes: &[u8], at: usize) -> Option<f32> {
    read_u32(bytes, at).map(f32::from_bits)
}

/// Fills a buffer front to back; every `put` fails with [`Error::BufferTooSmall`] instead of overrunning.
#[derive(Debug)]
pub(crate) struct Writer<'a> {
    rest: &'a mut [u8],
    written: usize,
}

impl<'a> Writer<'a> {
    pub(crate) const fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            rest: buffer,
            written: 0,
        }
    }

    /// Appends `bytes`.
    pub(crate) fn put(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let (head, tail) = core::mem::take(&mut self.rest)
            .split_at_mut_checked(bytes.len())
            .ok_or(Error::BufferTooSmall)?;
        head.copy_from_slice(bytes);
        self.rest = tail;
        self.written += bytes.len();
        Ok(())
    }

    /// The number of bytes written so far.
    pub(crate) const fn written(&self) -> usize {
        self.written
    }
}

#[cfg(test)]
mod tests;

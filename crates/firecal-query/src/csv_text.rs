//! Text written into a buffer the caller owns, with no allocation. `core` formats numbers the same way
//! everywhere, so the bytes are the same natively and in wasm.

use crate::QueryError;
use core::fmt::{Arguments, Error, Write};

/// A cursor over `out`; writing past its end is [`QueryError::BufferTooSmall`].
#[derive(Debug)]
pub(crate) struct TextBuffer<'a> {
    out: &'a mut [u8],
    written: usize,
}

impl<'a> TextBuffer<'a> {
    pub(crate) const fn new(out: &'a mut [u8]) -> Self {
        Self { out, written: 0 }
    }

    /// Appends the formatted text.
    pub(crate) fn put(&mut self, text: Arguments<'_>) -> Result<(), QueryError> {
        self.write_fmt(text).map_err(|_| QueryError::BufferTooSmall)
    }

    /// Appends `number` with `decimals` places, or nothing when it is not a finite number.
    pub(crate) fn put_number(&mut self, number: f64, decimals: usize) -> Result<(), QueryError> {
        if number.is_finite() {
            self.put(format_args!("{number:.decimals$}"))?;
        }
        Ok(())
    }

    /// How many bytes have been written.
    pub(crate) const fn written(&self) -> usize {
        self.written
    }
}

impl Write for TextBuffer<'_> {
    fn write_str(&mut self, text: &str) -> Result<(), Error> {
        let end = self.written.checked_add(text.len()).ok_or(Error)?;
        self.out
            .get_mut(self.written..end)
            .ok_or(Error)?
            .copy_from_slice(text.as_bytes());
        self.written = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests;

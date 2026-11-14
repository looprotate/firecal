//! Linear memory as the exports see it: addresses are `u32`, and an address that is not inside the memory is
//! an answer, never a trap.

#[cfg(target_arch = "wasm32")]
mod linear;
#[cfg(target_arch = "wasm32")]
pub(crate) use linear::LinearMemory;

/// A block of memory read and written by address. Address zero is never valid, as in C, so an unset pointer
/// is refused instead of read.
pub trait Memory {
    /// `len` bytes at `address`, or `None` when they are not all inside the memory.
    fn read(&self, address: u32, len: u32) -> Option<&[u8]>;
    /// `len` bytes at `address` to write, or `None` when they are not all inside the memory.
    fn write(&mut self, address: u32, len: u32) -> Option<&mut [u8]>;
}

/// Memory over a byte slice: address 1 is its first byte. What the tests and `cargo xtask parity` use in
/// place of the browser's linear memory.
#[derive(Debug)]
pub struct SliceMemory<'a> {
    bytes: &'a mut [u8],
}

impl<'a> SliceMemory<'a> {
    /// Wraps `bytes`; address 1 is `bytes[0]`.
    pub const fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes }
    }
}

/// The index range of `len` bytes at `address`, where address 1 is index 0.
fn index_range(address: u32, len: u32, total: usize) -> Option<core::ops::Range<usize>> {
    let start = usize::try_from(address.checked_sub(1)?).ok()?;
    let end = start.checked_add(usize::try_from(len).ok()?)?;
    (end <= total).then_some(start..end)
}

impl Memory for SliceMemory<'_> {
    fn read(&self, address: u32, len: u32) -> Option<&[u8]> {
        if len == 0 {
            return Some(&[]);
        }
        self.bytes.get(index_range(address, len, self.bytes.len())?)
    }

    fn write(&mut self, address: u32, len: u32) -> Option<&mut [u8]> {
        if len == 0 {
            return Some(&mut []);
        }
        let range = index_range(address, len, self.bytes.len())?;
        self.bytes.get_mut(range)
    }
}

#[cfg(test)]
mod tests;

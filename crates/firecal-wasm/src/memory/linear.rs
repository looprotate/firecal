//! The browser's linear memory, checked against its current size before every read and write.

use super::Memory;
use core::arch::wasm32::memory_size;

const BYTES_PER_PAGE: u64 = 65_536;

/// All of the module's linear memory.
#[derive(Debug)]
pub(crate) struct LinearMemory;

fn size_in_bytes() -> u64 {
    u64::try_from(memory_size::<0>()).unwrap_or(u64::MAX) * BYTES_PER_PAGE
}

/// The address and length as a `usize` pair, when the bytes are inside the memory and the address is not zero.
fn checked(address: u32, len: u32) -> Option<(usize, usize)> {
    if address == 0 || u64::from(address) + u64::from(len) > size_in_bytes() {
        return None;
    }
    Some((usize::try_from(address).ok()?, usize::try_from(len).ok()?))
}

impl Memory for LinearMemory {
    fn read(&self, address: u32, len: u32) -> Option<&[u8]> {
        if len == 0 {
            return Some(&[]);
        }
        let (start, count) = checked(address, len)?;
        // SAFETY: `checked` proved that `count` bytes from `start` are inside the linear memory, which
        // only grows and is never unmapped; `start` is not null; a `u8` has no alignment or validity rule;
        // and the returned borrow of `&self` stops `write` from being called while the slice lives.
        Some(unsafe { core::slice::from_raw_parts(start as *const u8, count) })
    }

    fn write(&mut self, address: u32, len: u32) -> Option<&mut [u8]> {
        if len == 0 {
            return Some(&mut []);
        }
        let (start, count) = checked(address, len)?;
        // SAFETY: as in `read`, and the borrow of `&mut self` makes this the only slice of the memory that
        // this module has alive.
        Some(unsafe { core::slice::from_raw_parts_mut(start as *mut u8, count) })
    }
}

//! Where the exports that fill a caller's buffer put their answer: a length in front of text, or a fixed block.

use crate::memory::Memory;
use crate::status::{INVALID, STATUS_OUTPUT_TOO_SMALL};

const LENGTH_BYTES: u32 = 4;

/// How many bytes of text fit in the buffer after its length, once the whole buffer is known to be readable.
pub(crate) fn text_capacity<M: Memory>(memory: &M, address: u32, len: u32) -> Result<usize, u32> {
    let capacity = len.checked_sub(LENGTH_BYTES).ok_or(INVALID)?;
    memory.read(address, len).ok_or(INVALID)?;
    usize::try_from(capacity).map_err(|_| INVALID)
}

/// Writes the length of `text` as a `u32` and then `text` into the buffer.
pub(crate) fn store_text<M: Memory>(
    memory: &mut M,
    address: u32,
    len: u32,
    text: &[u8],
) -> Result<(), u32> {
    let out = memory.write(address, len).ok_or(INVALID)?;
    let (length_out, text_out) = out
        .split_at_mut_checked(usize::try_from(LENGTH_BYTES).map_err(|_| INVALID)?)
        .ok_or(INVALID)?;
    length_out.copy_from_slice(
        &u32::try_from(text.len())
            .map_err(|_| INVALID)?
            .to_le_bytes(),
    );
    text_out
        .get_mut(..text.len())
        .ok_or(STATUS_OUTPUT_TOO_SMALL)?
        .copy_from_slice(text);
    Ok(())
}

/// The first `needed` bytes of the buffer, to be filled whole: [`STATUS_OUTPUT_TOO_SMALL`] when it is shorter,
/// [`INVALID`] when it is not all inside `memory`.
pub(crate) fn block<M: Memory>(
    memory: &mut M,
    address: u32,
    len: u32,
    needed: usize,
) -> Result<&mut [u8], u32> {
    let out = memory.write(address, len).ok_or(INVALID)?;
    out.get_mut(..needed).ok_or(STATUS_OUTPUT_TOO_SMALL)
}

/// Writes `numbers` as little-endian `f64` at the start of `out`.
pub(crate) fn put_doubles(out: &mut [u8], numbers: &[f64]) {
    for (slot, number) in out.as_chunks_mut::<8>().0.iter_mut().zip(numbers) {
        slot.copy_from_slice(&number.to_le_bytes());
    }
}

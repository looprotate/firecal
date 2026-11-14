//! Blocks of linear memory that the page asks for to put tiles and requests in, and gives back.

use std::sync::Mutex;

use crate::status::INVALID;

const LARGEST_BLOCK_BYTES: u32 = 1 << 30;

static BLOCKS: Mutex<Vec<(u32, Box<[u8]>)>> = Mutex::new(Vec::new());

/// A zeroed block of `len` bytes and its address, or [`INVALID`] for no bytes, more than 1 GiB, or no room.
/// It lives until [`free`] is called with its address.
pub(crate) fn allocate(len: u32) -> u32 {
    if len == 0 || len > LARGEST_BLOCK_BYTES {
        return INVALID;
    }
    let Ok(len) = usize::try_from(len) else {
        return INVALID;
    };
    let mut bytes = Vec::new();
    if bytes.try_reserve_exact(len).is_err() {
        return INVALID;
    }
    bytes.resize(len, 0);
    let block = bytes.into_boxed_slice();
    let Ok(address) = u32::try_from(block.as_ptr().addr()) else {
        return INVALID;
    };
    let Ok(mut blocks) = BLOCKS.lock() else {
        return INVALID;
    };
    blocks.push((address, block));
    address
}

/// Gives back the block at `address`. [`INVALID`] when no such block is held, so a second free is harmless.
pub(crate) fn free(address: u32) -> u32 {
    let Ok(mut blocks) = BLOCKS.lock() else {
        return INVALID;
    };
    blocks
        .iter()
        .position(|(held, _)| *held == address)
        .map_or(INVALID, |at| {
            blocks.swap_remove(at);
            0
        })
}

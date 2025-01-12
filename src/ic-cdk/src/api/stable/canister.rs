use super::*;

/// A standard implementation of [`StableMemory`].
///
/// Useful for creating [`StableWriter`] and [`StableReader`].
#[derive(Default)]
pub struct CanisterStableMemory {}

impl StableMemory for CanisterStableMemory {
    fn stable_size(&self) -> u32 {
        // SAFETY: ic0.stable_size is always safe to call.
        unsafe { ic0::stable_size() as u32 }
    }

    fn stable64_size(&self) -> u64 {
        // SAFETY: ic0.stable64_size is always safe to call.
        unsafe { ic0::stable64_size() as u64 }
    }

    fn stable_grow(&self, new_pages: u32) -> Result<u32, StableMemoryError> {
        // SAFETY: ic0.stable_grow is always safe to call.
        unsafe {
            match ic0::stable_grow(new_pages as i32) {
                -1 => Err(StableMemoryError::OutOfMemory),
                x => Ok(x as u32),
            }
        }
    }

    fn stable64_grow(&self, new_pages: u64) -> Result<u64, StableMemoryError> {
        // SAFETY: ic0.stable64_grow is always safe to call.
        unsafe {
            match ic0::stable64_grow(new_pages as i64) {
                -1 => Err(StableMemoryError::OutOfMemory),
                x => Ok(x as u64),
            }
        }
    }

    fn stable_write(&self, offset: u32, buf: &[u8]) {
        // SAFETY: `buf`, being &[u8], is a readable sequence of bytes, and therefore valid to pass to ic0.stable_write.
        unsafe {
            ic0::stable_write(offset as i32, buf.as_ptr() as i32, buf.len() as i32);
        }
    }

    fn stable64_write(&self, offset: u64, buf: &[u8]) {
        // SAFETY: `buf`, being &[u8], is a readable sequence of bytes, and therefore valid to pass to ic0.stable64_write.
        unsafe {
            ic0::stable64_write(offset as i64, buf.as_ptr() as i64, buf.len() as i64);
        }
    }

    fn stable_read(&self, offset: u32, buf: &mut [u8]) {
        // SAFETY: `buf`, being &mut [u8], is a writable sequence of bytes, and therefore valid to pass to ic0.stable_read.
        unsafe {
            ic0::stable_read(buf.as_ptr() as i32, offset as i32, buf.len() as i32);
        }
    }

    fn stable64_read(&self, offset: u64, buf: &mut [u8]) {
        // SAFETY: `buf`, being &mut [u8], is a writable sequence of bytes, and therefore valid to pass to ic0.stable64_read.
        unsafe {
            ic0::stable64_read(buf.as_ptr() as i64, offset as i64, buf.len() as i64);
        }
    }
}

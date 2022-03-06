#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use libc::*;

pub use encode::encode_silk;

mod encode;

#[allow(dead_code)]
pub(crate) mod sdk {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) struct CMemory {
    ptr: *mut c_void,
}

impl CMemory {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            ptr: unsafe { malloc(size) },
        }
    }
}

impl Drop for CMemory {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encode_silk;

    #[test]
    fn test_encode() {
        let input = std::fs::read("test.pcm").unwrap();
        let output = encode_silk(input, 24000, 24000, true).unwrap();
        std::fs::write("output.silk", output).unwrap();
    }
}
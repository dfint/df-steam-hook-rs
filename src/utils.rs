use std::ffi::CStr;
use std::slice;
use std::str::Utf8Error;

pub unsafe fn cstr_from_bytes(src: *const u8, size: usize) -> Result<&'static str, Utf8Error> {
  CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(src, size + 1)).to_str()
}

use std::ptr;
use winapi::um::libloaderapi::GetModuleHandleW;

lazy_static! {
  static ref MODULE: usize = unsafe { GetModuleHandleW(ptr::null()) as usize };
}

#[allow(dead_code)]
pub fn address(offset: usize) -> usize {
  *MODULE + offset
}

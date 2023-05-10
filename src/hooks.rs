use detour::static_detour;
use log::trace;
use std::error::Error;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::slice;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use crate::utils;
use r#macro::attach;

pub unsafe fn attach_all() -> Result<(), Box<dyn Error>> {
  attach_menu_interface_loop()?;
  attach_string_copy_n()?;
  Ok(())
}

#[attach(fastcall)]
fn menu_interface_loop(a1: usize) {
  unsafe { original!(a1) };
  trace!("MENU");
}

#[attach(cdecl)]
fn string_copy_n(dst: *mut c_char, src: *const u8, size: usize) -> *mut c_char {
  unsafe {
    if size <= 1 {
      return original!(dst, src, size);
    }
    match CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(src, size + 1)).to_str() {
      Ok(value) => match DICTIONARY.get(value) {
        Some(translate) => original!(dst, translate.as_ptr(), translate.len()),
        _ => original!(dst, src, size),
      },
      _ => original!(dst, src, size),
    }
  }
}

use detour::static_detour;
use log::trace;
use std::error::Error;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;
use std::{mem, ptr};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use r#macro::attach;

lazy_static! {
  static ref MODULE: usize = unsafe { GetModuleHandleW(ptr::null()) as usize };
}

// static_detour! {
// static handle_menu_interface_loop: unsafe extern "fastcall" fn(usize);
// static handle_string_copy_n: unsafe extern "cdecl" fn(*mut c_char,*const u8, usize) -> *mut c_char;
// }

pub fn address(offset: usize) -> usize {
  *MODULE + offset
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
      return handle_string_copy_n.call(dst, src, size);
    }

    let v = slice::from_raw_parts(src, size + 1);
    match CStr::from_bytes_with_nul_unchecked(v).to_str() {
      Ok(value) => match DICTIONARY.get(value) {
        Some(translate) => {
          return handle_string_copy_n.call(dst, translate.as_ptr(), translate.len())
        }
        _ => (),
      },
      _ => (),
    }

    original!(dst, src, size)
  }
}

// pub unsafe fn attach_menu_hook() -> Result<(), Box<dyn Error>> {
//   let target = mem::transmute(address(CONFIG.offset.menu_interface_loop));

//   handle_menu_interface_loop
//     .initialize(target, |a1: usize| {
//       unsafe { handle_menu_interface_loop.call(a1) }
//       trace!("MENU");
//     })?
//     .enable()?;
//   Ok(())
// }

// pub unsafe fn attach_string_copy_n() -> Result<(), Box<dyn Error>> {
//   let target = mem::transmute(address(CONFIG.offset.string_copy_n));

//   handle_string_copy_n
//     .initialize(target, string_copy_n)?
//     .enable()?;
//   Ok(())
// }

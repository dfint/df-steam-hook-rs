use detour::static_detour;
use std::error::Error;
use std::mem;
use std::os::raw::c_char;

use crate::config::CONFIG;
use crate::cxxstring::CxxString;
use crate::dictionary::DICTIONARY;
use crate::utils;

use r#macro::attach;

pub unsafe fn attach_all() -> Result<(), Box<dyn Error>> {
  if CONFIG.settings.enable_translation {
    attach_string_copy_n()?;
    attach_string_append_n()?;
    attach_addst()?;
    attach_addst_top()?;
    attach_addst_flag()?;
  }
  Ok(())
}

#[attach(cdecl)]
fn string_copy_n(dst: *mut c_char, src: *const u8, size: usize) -> *mut c_char {
  unsafe {
    match (utils::cstr(src, size + 1), size > 1) {
      (Ok(value), true) => match DICTIONARY.get(value) {
        Some(translate) => original!(dst, translate.as_ptr(), translate.len()),
        _ => original!(dst, src, size),
      },
      (_, _) => original!(dst, src, size),
    }
  }
}

#[attach(cdecl)]
fn string_append_n(dst: *mut c_char, src: *const u8, size: usize) -> *mut c_char {
  unsafe {
    match (utils::cstr(src, size + 1), size > 1) {
      (Ok(value), true) => match DICTIONARY.get(value) {
        Some(translate) => original!(dst, translate.as_ptr(), translate.len()),
        _ => original!(dst, src, size),
      },
      (_, _) => original!(dst, src, size),
    }
  }
}

#[attach(fastcall)]
fn addst(gps: usize, src: *const u8, justify: u8, space: u32) {
  unsafe {
    let s: &mut CxxString = std::mem::transmute(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let mut cxxstr = CxxString::new(translate.clone().as_mut_ptr(), translate.len());
          original!(gps, cxxstr.as_ptr(), justify, space)
        }
        _ => original!(gps, src, justify, space),
      },
      _ => original!(gps, src, justify, space),
    }
  }
}

#[attach(fastcall)]
fn addst_top(gps: usize, src: *const u8, a3: usize) {
  unsafe {
    let s: &mut CxxString = std::mem::transmute(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let mut cxxstr = CxxString::new(translate.clone().as_mut_ptr(), translate.len());
          original!(gps, cxxstr.as_ptr(), a3)
        }
        _ => original!(gps, src, a3),
      },
      _ => original!(gps, src, a3),
    }
  }
}

#[attach(fastcall)]
fn addst_flag(gps: usize, src: *const u8, a3: usize, a4: usize, flag: u32) {
  unsafe {
    let s: &mut CxxString = std::mem::transmute(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let mut cxxstr = CxxString::new(translate.clone().as_mut_ptr(), translate.len());
          original!(gps, cxxstr.as_ptr(), a3, a4, flag)
        }
        _ => original!(gps, src, a3, a4, flag),
      },
      _ => original!(gps, src, a3, a4, flag),
    }
  }
}

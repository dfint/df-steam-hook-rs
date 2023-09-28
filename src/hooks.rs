use retour::static_detour;
use std::error::Error;
use std::mem;
use std::os::raw::c_char;

use crate::config::CONFIG;
use crate::cxxset::CxxSet;
use crate::cxxstring::CxxString;
use crate::dictionary::DICTIONARY;
use crate::utils;

use r#macro::hook;

pub unsafe fn attach_all() -> Result<(), Box<dyn Error>> {
  if CONFIG.settings.enable_translation {
    attach_string_copy_n()?;
    attach_string_append_n()?;
    attach_addst()?;
    attach_addst_top()?;
    attach_addst_flag()?;
  }
  if CONFIG.settings.enable_search {
    attach_standardstringentry()?;
    attach_simplify_string()?;
    attach_upper_case_string()?;
    attach_lower_case_string()?;
    attach_capitalize_string_words()?;
    attach_capitalize_string_first_word()?;
  }
  // attach_string_copy()?;
  // attach_string_append()?;
  Ok(())
}

pub unsafe fn detach_all() -> Result<(), Box<dyn Error>> {
  if CONFIG.settings.enable_translation {
    detach_string_copy_n()?;
    detach_string_append_n()?;
    detach_addst()?;
    detach_addst_top()?;
    detach_addst_flag()?;
  }
  if CONFIG.settings.enable_search {
    detach_standardstringentry()?;
    detach_simplify_string()?;
    detach_upper_case_string()?;
    detach_lower_case_string()?;
    detach_capitalize_string_words()?;
    detach_capitalize_string_first_word()?;
  }
  Ok(())
}

#[cfg_attr(target_os = "windows", hook(bypass))]
#[cfg_attr(target_os = "linux", hook(module = "libc.so.6", symbol = "__strcpy_chk"))]
fn string_copy(dst: *mut c_char, src: *mut u8) -> *const c_char {
  unsafe {
    let len = libc::strnlen(src as *const c_char, 1000);
    match (std::ffi::CStr::from_ptr(src as *const c_char).to_str(), len > 1) {
      (Ok(value), true) => match DICTIONARY.get(value) {
        Some(translate) => {
          let (ptr, _, _) = translate.to_owned().into_raw_parts();
          original!(dst, ptr)
        }
        _ => original!(dst, src),
      },
      (_, _) => original!(dst, src),
    }
  }
}

#[cfg_attr(target_os = "windows", hook(bypass))]
#[cfg_attr(target_os = "linux", hook(module = "libc.so.6", symbol = "__strcat_chk"))]
fn string_append(dst: *mut c_char, src: *mut u8) -> *const c_char {
  unsafe {
    let len = libc::strnlen(src as *const c_char, 1000);
    match (std::ffi::CStr::from_ptr(src as *const c_char).to_str(), len > 1) {
      (Ok(value), true) => match DICTIONARY.get(value) {
        Some(translate) => {
          let (ptr, _, _) = translate.to_owned().into_raw_parts();
          original!(dst, ptr)
        }
        _ => original!(dst, src),
      },
      (_, _) => original!(dst, src),
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
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

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
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

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(
  target_os = "linux",
  hook(
    module = "libg_src_lib.so",
    symbol = "_ZN9graphicst5addstERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE13justificationi"
  )
)]
fn addst(gps: usize, src: *const u8, justify: u8, space: u32) {
  unsafe {
    let s = CxxString::from_ptr(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let (ptr, len, _) = translate.to_owned().into_raw_parts();
          let mut cxxstr = CxxString::new(ptr, len);
          #[cfg(target_os = "linux")]
          {
            if cxxstr.len < 16 {
              cxxstr.ptr = cxxstr.sso.buf.as_mut_ptr();
            }
          }
          original!(gps, cxxstr.as_ptr(), justify, space)
        }
        _ => original!(gps, src, justify, space),
      },
      _ => original!(gps, src, justify, space),
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(
  target_os = "linux",
  hook(
    module = "libg_src_lib.so",
    symbol = "_ZN9graphicst9top_addstERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE13justificationi"
  )
)]
fn addst_top(gps: usize, src: *const u8, a3: usize) {
  unsafe {
    let s = CxxString::from_ptr(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let (ptr, len, _) = translate.to_owned().into_raw_parts();
          let mut cxxstr = CxxString::new(ptr, len);
          #[cfg(target_os = "linux")]
          {
            if cxxstr.len < 16 {
              cxxstr.ptr = cxxstr.sso.buf.as_mut_ptr();
            }
          }
          original!(gps, cxxstr.as_ptr(), a3)
        }
        _ => original!(gps, src, a3),
      },
      _ => original!(gps, src, a3),
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(
  target_os = "linux",
  hook(
    module = "libg_src_lib.so",
    symbol = "_ZN9graphicst10addst_flagERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE13justificationij"
  )
)]
fn addst_flag(gps: usize, src: *const u8, a3: usize, a4: usize, flag: u32) {
  unsafe {
    let s = CxxString::from_ptr(src);
    match s.to_str() {
      Ok(converted) => match DICTIONARY.get(converted) {
        Some(translate) => {
          let (ptr, len, _) = translate.to_owned().into_raw_parts();
          let mut cxxstr = CxxString::new(ptr, len);
          #[cfg(target_os = "linux")]
          {
            if cxxstr.len < 16 {
              cxxstr.ptr = cxxstr.sso.buf.as_mut_ptr();
            }
          }
          original!(gps, cxxstr.as_ptr(), a3, a4, flag)
        }
        _ => original!(gps, src, a3, a4, flag),
      },
      _ => original!(gps, src, a3, a4, flag),
    }
  }
}

#[non_exhaustive]
struct StringEntry;

impl StringEntry {
  pub const LETTERS: u8 = 1;
  pub const SPACE: u8 = 2;
  pub const NUMBERS: u8 = 4;
  pub const CAPS: u8 = 8;
  pub const SYMBOLS: u8 = 16;
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn standardstringentry(src: *const u8, maxlen: i64, flag: u8, events_ptr: *const u8) -> i32 {
  unsafe {
    let content = CxxString::from_ptr(src);
    let events = CxxSet::<u32>::from_ptr(events_ptr);
    let shift = CONFIG.offset.keybinding.unwrap_or(0);
    let mut entry = shift + 1;
    let mut ranges = vec![shift..=shift];

    if (flag & StringEntry::SYMBOLS) > 0 {
      ranges.push(shift..=(shift + 255));
    }
    if (flag & StringEntry::LETTERS) > 0 {
      ranges.push((shift + 65)..=(shift + 90));
      ranges.push((shift + 97)..=(shift + 122));
      ranges.push((shift + 192)..=(shift + 255));
    }
    if (flag & StringEntry::SPACE) > 0 {
      ranges.push((shift + 32)..=(shift + 32));
    }
    if (flag & StringEntry::NUMBERS) > 0 {
      ranges.push((shift + 48)..=(shift + 57));
    }

    'outer: for range in ranges.into_iter() {
      'inner: for item in range.into_iter() {
        if events.contains(item) {
          entry = item;
          if item > shift + 192 && item <= shift + 255 {
            entry += 1;
          }
          break 'outer;
        }
      }
    }

    match entry - shift {
      1 => return 0,
      0 => {
        if content.len > 0 {
          content.resize(content.len - 1);
        }
      }
      value => {
        let mut cursor = content.len;
        if cursor >= maxlen as usize {
          cursor = (maxlen - 1) as usize;
        }
        if cursor < 0 {
          cursor = 0;
        }
        if content.len < cursor + 1 {
          content.resize(cursor + 1);
        }
        let letter = match flag & StringEntry::CAPS > 0 {
          true => capitalize(value as u8),
          false => value as u8,
        };
        content[cursor] = letter;
      }
    }
    events.clear();
    1
  }
}

fn capitalize(symbol: u8) -> u8 {
  match symbol {
    symbol if symbol >= 97 && symbol <= 122 => symbol - 32, // latin
    symbol if symbol >= 224 => symbol - 32,                 // cyrillic
    symbol if symbol == 184 => 168,                         // cyrillic ё
    _ => symbol,
  }
}

fn lowercast(symbol: u8) -> u8 {
  match symbol {
    symbol if symbol >= 65 && symbol <= 90 => symbol + 32,   // latin
    symbol if symbol >= 192 && symbol <= 223 => symbol + 32, // cyrillic
    symbol if symbol == 168 => 184,                          // cyrillic ё
    _ => symbol,
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn simplify_string(src: *const u8) {
  unsafe {
    let mut content = CxxString::from_ptr(src);
    for i in 0..content.len {
      content[i] = match lowercast(content[i]) {
        129 | 150 | 151 | 154 | 163 => 117,
        152 => 121,
        164 | 165 => 110,
        131..=134 | 142 | 143 | 145 | 146 | 160 => 97,
        130 | 136..=138 | 144 => 101,
        139..=141 | 161 => 105,
        147..=149 | 153 | 162 => 111,
        128 | 135 => 99,
        value => value,
      };
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn upper_case_string(src: *const u8) {
  unsafe {
    let mut content = CxxString::from_ptr(src);
    for i in 0..content.len {
      content[i] = match capitalize(content[i]) {
        129 => 154,
        164 => 165,
        132 => 142,
        134 => 143,
        130 => 144,
        148 => 153,
        135 => 128,
        145 => 146,
        value => value,
      };
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn lower_case_string(src: *const u8) {
  unsafe {
    let mut content = CxxString::from_ptr(src);
    for i in 0..content.len {
      content[i] = match lowercast(content[i]) {
        154 => 129,
        165 => 164,
        142 => 132,
        143 => 134,
        144 => 130,
        153 => 148,
        128 => 135,
        146 => 145,
        value => value,
      };
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn capitalize_string_words(src: *const u8) {
  unsafe {
    let mut content = CxxString::from_ptr(src);
    for i in 0..content.len {
      let mut conf = false;
      if (i > 0 && content[i - 1] == 32 || content[i - 1] == 34)
        || (i >= 2 && content[i - 1] == 39 && (content[i - 2] == 32 || content[i - 2] == 44))
      {
        conf = true;
      }
      if i == 0 || conf {
        content[i] = match capitalize(content[i]) {
          129 => 154,
          164 => 165,
          132 => 142,
          134 => 143,
          130 => 144,
          148 => 153,
          135 => 128,
          145 => 146,
          value => value,
        };
      }
    }
  }
}

#[cfg_attr(target_os = "windows", hook)]
#[cfg_attr(target_os = "linux", hook(bypass))]
fn capitalize_string_first_word(src: *const u8) {
  unsafe {
    let mut content = CxxString::from_ptr(src);
    for i in 0..content.len {
      let mut conf = false;
      if (i > 0 && content[i - 1] == 32 || content[i - 1] == 34)
        || (i >= 2 && content[i - 1] == 39 && (content[i - 2] == 32 || content[i - 2] == 44))
      {
        conf = true;
      }
      if i == 0 || conf {
        content[i] = match capitalize(content[i]) {
          129 => 154,
          164 => 165,
          132 => 142,
          134 => 143,
          130 => 144,
          148 => 153,
          135 => 128,
          145 => 146,
          value => value,
        };
        if content[i] != 32 && content[i] != 34 {
          return;
        }
      }
    }
  }
}

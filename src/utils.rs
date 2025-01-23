use dlopen2::raw::Library;

use crate::constants::PATH_SDL2;

#[cfg(target_os = "windows")]
#[static_init::dynamic]
pub static MODULE: usize = unsafe { winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) as usize };

#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static MODULE: usize = 0;

#[static_init::dynamic]
static SDL_MESSAGE_BOX: fn(u32, *const i8, *const i8, *const u8) -> i32 =
  unsafe { symbol_handle::<fn(u32, *const i8, *const i8, *const u8) -> i32>(PATH_SDL2, "SDL_ShowSimpleMessageBox") };

#[static_init::dynamic]
static SDL_ERROR: fn() -> *const i8 = unsafe { symbol_handle::<fn() -> *const i8>(PATH_SDL2, "SDL_GetError") };

pub unsafe fn symbol_handle<T>(module: &str, symbol: &str) -> T {
  if module == "self" {
    return symbol_handle_self::<T>(symbol);
  }
  let lib = Library::open(module).expect("Could not open library");
  unsafe { lib.symbol(symbol) }.unwrap()
}

pub unsafe fn symbol_handle_self<T>(symbol: &str) -> T {
  let lib = Library::open_self().expect("Could not open self");
  unsafe { lib.symbol(symbol) }.unwrap()
}

#[allow(dead_code)]
pub fn address(offset: usize) -> usize {
  *MODULE + offset
}

#[allow(dead_code)]
#[repr(u32)]
pub enum MessageIconType {
  Error = 0x10,
  Warning = 0x20,
  Info = 0x40,
}

#[allow(dangling_pointers_from_temporaries)]
pub fn message_box(title: &str, text: &str, icon: MessageIconType) {
  let ret = SDL_MESSAGE_BOX(
    icon as u32,
    std::ffi::CString::new(title).unwrap().as_ptr(),
    std::ffi::CString::new(text).unwrap().as_ptr(),
    std::ptr::null(),
  );
  if ret == -1 {
    log::error!("SDL_ShowSimpleMessageBox: {}", unsafe {
      std::ffi::CStr::from_ptr(SDL_ERROR()).to_str().unwrap()
    });
  }
}

#[allow(dead_code)]
pub unsafe fn cstr<T>(src: *const T, size: usize) -> Result<&'static str, std::str::Utf8Error> {
  std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(src as *const u8, size)).to_str()
}

pub fn log_level(level: usize) -> log::LevelFilter {
  match level {
    0 => log::LevelFilter::Trace,
    1 => log::LevelFilter::Debug,
    2 => log::LevelFilter::Info,
    3 => log::LevelFilter::Warn,
    4 => log::LevelFilter::Error,
    5 => log::LevelFilter::Off,
    _ => log::LevelFilter::Info,
  }
}

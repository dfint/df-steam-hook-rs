use dlopen::raw::Library;

#[cfg(target_os = "windows")]
const SDL2: &'static str = "SDL2.dll";
#[cfg(target_os = "linux")]
const SDL2: &'static str = "libSDL2-2.0.so.0";

#[cfg(target_os = "windows")]
#[static_init::dynamic]
pub static MODULE: usize = unsafe { winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) as usize };

#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static MODULE: usize = unsafe { libc::dlopen(std::ptr::null(), libc::RTLD_NOW) as usize };

#[static_init::dynamic]
static SDL_MESSAGE_BOX: fn(u32, *const i8, *const i8, *const u8) -> i32 =
  unsafe { symbol_handle::<fn(u32, *const i8, *const i8, *const u8) -> i32>(SDL2, "SDL_ShowSimpleMessageBox") };

#[static_init::dynamic]
static SDL_ERROR: fn() -> *const i8 = unsafe { symbol_handle::<fn() -> *const i8>(SDL2, "SDL_GetError") };

pub unsafe fn symbol_handle<T>(module: &str, symbol: &str) -> T {
  let lib = Library::open(module).expect("Could not open library");
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

#[allow(temporary_cstring_as_ptr)]
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
pub unsafe fn cstr(src: *const u8, size: usize) -> Result<&'static str, std::str::Utf8Error> {
  std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(src, size)).to_str()
}

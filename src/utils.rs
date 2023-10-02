use std::collections::HashMap;

use dlopen2::raw::Library;

#[cfg(target_os = "windows")]
const SDL2: &'static str = "SDL2.dll";
#[cfg(target_os = "linux")]
const SDL2: &'static str = "libSDL2-2.0.so.0";

#[cfg(target_os = "windows")]
#[static_init::dynamic]
pub static MODULE: usize = unsafe { winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) as usize };

#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static MODULE: usize = 0;

#[static_init::dynamic]
static SDL_MESSAGE_BOX: fn(u32, *const i8, *const i8, *const u8) -> i32 =
  unsafe { symbol_handle::<fn(u32, *const i8, *const i8, *const u8) -> i32>(SDL2, "SDL_ShowSimpleMessageBox") };

#[static_init::dynamic]
static SDL_ERROR: fn() -> *const i8 = unsafe { symbol_handle::<fn() -> *const i8>(SDL2, "SDL_GetError") };

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
pub unsafe fn cstr<T>(src: *const T, size: usize) -> Result<&'static str, std::str::Utf8Error> {
  std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(src as *const u8, size)).to_str()
}

#[static_init::dynamic]
pub static UTF_TO_CP1251: std::collections::HashMap<u32, u8> = HashMap::from([
  (37072, 192),
  (37328, 193),
  (37584, 194),
  (37840, 195),
  (38096, 196),
  (38352, 197),
  (38608, 198),
  (38864, 199),
  (39120, 200),
  (39376, 201),
  (39632, 202),
  (39888, 203),
  (40144, 204),
  (40400, 205),
  (40656, 206),
  (40912, 207),
  (41168, 208),
  (41424, 209),
  (41680, 210),
  (41936, 211),
  (42192, 212),
  (42448, 213),
  (42704, 214),
  (42960, 215),
  (43216, 216),
  (43472, 217),
  (43728, 218),
  (43984, 219),
  (44240, 220),
  (44496, 221),
  (44752, 222),
  (45008, 223),
  (45264, 224),
  (45520, 225),
  (45776, 226),
  (46032, 227),
  (46288, 228),
  (46544, 229),
  (46800, 230),
  (47056, 231),
  (47312, 232),
  (47568, 233),
  (47824, 234),
  (48080, 235),
  (48336, 236),
  (48592, 237),
  (48848, 238),
  (49104, 239),
  (32977, 240),
  (33233, 241),
  (33489, 242),
  (33745, 243),
  (34001, 244),
  (34257, 245),
  (34513, 246),
  (34769, 247),
  (35025, 248),
  (35281, 249),
  (35537, 250),
  (35793, 251),
  (36049, 252),
  (36305, 253),
  (36561, 254),
  (36817, 255),
  (33232, 168),
  (37329, 184),
  (34000, 170),
  (38097, 186),
  (34768, 175),
  (38865, 191),
  (34512, 178),
  (38609, 179),
  (37074, 165),
  (37330, 180),
  (36560, 161),
  (40657, 162),
]);

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

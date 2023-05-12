use std::ptr;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::MessageBoxW;

lazy_static! {
  static ref MODULE: usize = unsafe { GetModuleHandleW(ptr::null()) as usize };
}

#[allow(dead_code)]
pub fn address(offset: usize) -> usize {
  *MODULE + offset
}

#[allow(dead_code)]
pub enum MessageIconType {
  Error = 16,
  Info = 64,
  Question = 66,
  Warning = 58,
}

#[allow(dead_code)]
pub unsafe fn message_box(message: &str, caption: &str, icon: MessageIconType) {
  let message = message.encode_utf16();
  let mut message_vec: Vec<u16> = message.collect();
  message_vec.push(0);
  let caption = caption.encode_utf16();
  let mut caption_vec: Vec<u16> = caption.collect();
  caption_vec.push(0);
  MessageBoxW(
    ptr::null_mut(),
    message_vec.as_ptr(),
    caption_vec.as_ptr(),
    icon as u32,
  );
}

pub unsafe fn cstr(src: *const u8, size: usize) -> Result<&'static str, std::str::Utf8Error> {
  std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(src, size)).to_str()
}

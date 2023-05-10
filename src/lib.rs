#[macro_use(lazy_static)]
extern crate lazy_static;

use log::trace;
use log::LevelFilter;
// // use detour::static_detour;
use std::error::Error;
// use std::fs::File;
// use std::{ffi::CString, iter, mem};
// use winapi::ctypes::{c_int, c_void};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
// use winapi::shared::windef::HWND;
// use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use crate::config::CONFIG;

mod config;
mod dictionary;
mod hooks;

#[no_mangle]
pub unsafe extern "system" fn DllMain(
  _module: HINSTANCE,
  call_reason: DWORD,
  _reserved: LPVOID,
) -> BOOL {
  match call_reason {
    DLL_PROCESS_ATTACH => attach().is_ok() as BOOL,
    DLL_PROCESS_DETACH => detach().is_ok() as BOOL,
    _ => TRUE,
  }
}

fn attach() -> Result<(), Box<dyn Error>> {
  simple_logging::log_to_file("test.log", LevelFilter::Trace)?;
  trace!("{:}", CONFIG.metadata.name);
  unsafe {
    hooks::attach_all()?;
  }
  trace!("attached");
  Ok(())
}

fn detach() -> Result<(), Box<dyn Error>> {
  trace!("detached");
  Ok(())
}

#[macro_use(lazy_static)]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod crash;
mod cxxset;
mod cxxstring;
mod dictionary;
mod hooks;
mod utils;

use log::LevelFilter;
use log::{error, info, trace};
use std::error::Error;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use crate::config::CONFIG;

#[no_mangle]
pub unsafe extern "system" fn DllMain(
  _module: HINSTANCE,
  call_reason: DWORD,
  _reserved: LPVOID,
) -> BOOL {
  crash::install();
  match call_reason {
    DLL_PROCESS_ATTACH => attach().is_ok() as BOOL,
    DLL_PROCESS_DETACH => detach().is_ok() as BOOL,
    _ => TRUE,
  }
}

fn attach() -> Result<(), Box<dyn Error>> {
  simple_logging::log_to_file(&CONFIG.settings.log_file, LevelFilter::Trace)?;
  if CONFIG.metadata.name != "dfint localization hook" {
    error!("unable to find config file");
    unsafe {
      utils::message_box(
        "unable to find config file",
        "dfint hook error",
        utils::MessageIconType::Error,
      );
    }
    std::process::exit(2);
  }
  info!("pe checksum: 0x{:x}", CONFIG.offset_metadata.checksum);
  info!("offsets version: {}", CONFIG.offset_metadata.version);
  unsafe {
    hooks::attach_all()?;
  }
  trace!("hooks attached");
  Ok(())
}

fn detach() -> Result<(), Box<dyn Error>> {
  trace!("hooks detached");
  Ok(())
}

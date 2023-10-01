#![feature(vec_into_raw_parts)]

#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
// mod crash;
mod cxxset;
mod cxxstring;
mod dictionary;
mod hooks;
mod utils;
mod watchdog;

use log::LevelFilter;
use log::{error, info, trace};

use crate::config::CONFIG;

#[static_init::constructor]
#[no_mangle]
extern "C" fn attach() {
  std::env::set_var("RUST_BACKTRACE", "1");
  // unsafe {
  //   crash::install();
  // }
  if CONFIG.settings.watchdog {
    watchdog::install();
  }
  simple_logging::log_to_file(&CONFIG.settings.log_file, LevelFilter::Trace).unwrap();
  if CONFIG.metadata.name != "dfint localization hook" {
    error!("unable to find config file");
    utils::message_box(
      "unable to find config file",
      "dfint hook error",
      utils::MessageIconType::Error,
    );
    std::process::exit(2);
  }
  info!("pe checksum: 0x{:x}", CONFIG.offset_metadata.checksum);
  info!("offsets version: {}", CONFIG.offset_metadata.version);
  info!("hook version: {}", CONFIG.hook_version);
  unsafe {
    hooks::attach_all().unwrap();
  }
  trace!("hooks attached");
}

#[static_init::destructor]
#[no_mangle]
extern "C" fn detach() {
  unsafe {
    match hooks::disable_all() {
      _ => (),
    };
  }
  trace!("hooks detached");
}

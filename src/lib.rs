#![feature(vec_into_raw_parts)]

#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod constants;
mod cxxstring;
mod dictionary;
mod encoding;
mod hooks;
mod utils;
mod watchdog;

use log::{debug, error, info};

use crate::config::CONFIG;
use crate::constants::PATH_DICTIONARY;
use crate::dictionary::DICTIONARY;

#[static_init::constructor]
#[no_mangle]
extern "C" fn attach() {
  std::env::set_var("RUST_BACKTRACE", "1");
  simple_logging::log_to_file(&CONFIG.settings.log_file, utils::log_level(CONFIG.settings.log_level)).unwrap();
  if CONFIG.metadata.name != "dfint localization hook" {
    error!("unable to find config file");
    utils::message_box(
      "dfint hook error",
      "Unable to find config file, translation unavaible",
      utils::MessageIconType::Error,
    );
    return;
  }
  info!("pe checksum: 0x{:x}", CONFIG.offset_metadata.checksum);
  info!("offsets version: {}", CONFIG.offset_metadata.version);
  info!("hook version: {}", CONFIG.hook_version);
  info!("dictionary \"{}\", items {}", PATH_DICTIONARY, DICTIONARY.size());
  if CONFIG.offset_metadata.name != "not found" {
    match unsafe { hooks::attach_all() } {
      Ok(_) => debug!("hooks attached"),
      Err(err) => {
        error!("unable to attach hooks, {:?}", err);
        utils::message_box(
          "dfint hook error",
          "Unable to attach hooks, translation unavaible",
          utils::MessageIconType::Error,
        );
        return;
      }
    };
    if CONFIG.settings.watchdog {
      watchdog::install();
    }
  }
}

#[static_init::destructor]
#[no_mangle]
extern "C" fn detach() {
  unsafe {
    watchdog::uninstall();
    let _ = hooks::disable_all();
    debug!("hooks detached");
  }
}

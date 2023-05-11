use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use toml::{map::Map, Table, Value};

lazy_static! {
  static ref PATH_PREFIX: String = String::from("./dfint_data/");
  pub static ref CONFIG: Config = Config::new(PATH_PREFIX.clone()).unwrap();
}

pub struct Config {
  pub metadata: Metadata,
  pub settings: Settings,
  pub offset: Offset,
}

pub struct Metadata {
  pub name: String,
}

pub struct Settings {
  pub log_level: u32,
  pub log_file: String,
  pub crash_report: bool,
  pub crash_report_dir: String,
  pub enable_search: bool,
  pub enable_translation: bool,
  pub enable_patches: bool,
  pub dictionary: String,
  pub watchdog: bool,
}

pub struct Offset {
  pub string_copy_n: usize,
  pub menu_interface_loop: usize,
}

impl Config {
  pub fn new(path: String) -> Result<Self, Box<dyn Error>> {
    let config = Self::read_toml(path + "dfint_config.toml")?;
    let metadata = config["metadata"].as_table().unwrap();
    let settings = config["settings"].as_table().unwrap();

    Ok(Self {
      metadata: Metadata {
        name: String::from(metadata["name"].as_str().unwrap()),
      },
      settings: Settings {
        log_level: 1,
        log_file: String::from(settings["log_file"].as_str().unwrap()),
        crash_report: settings["crash_report"].as_bool().unwrap(),
        crash_report_dir: String::from(settings["crash_report_dir"].as_str().unwrap()),
        enable_search: settings["enable_search"].as_bool().unwrap(),
        enable_translation: settings["enable_translation"].as_bool().unwrap(),
        enable_patches: settings["enable_patches"].as_bool().unwrap(),
        dictionary: String::from(settings["dictionary"].as_str().unwrap()),
        watchdog: settings["watchdog"].as_bool().unwrap(),
      },
      offset: Offset {
        string_copy_n: 0xB5D0,
        menu_interface_loop: 0x167890,
      },
    })
  }

  fn read_toml(filename: String) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let result = content.parse::<Table>()?;
    Ok(result)
  }
}

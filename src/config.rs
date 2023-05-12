use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, path::Path};

use exe::{VecPE, PE};
use toml::{map::Map, Table, Value};
use walkdir::WalkDir;

use crate::utils;

static EXE_FILE: &str = "./Dwarf Fortress.exe";
static CONFIG_FILE: &str = "./dfint_data/dfint_config.toml";
static OFFSETS_DIR: &str = "./dfint_data/offsets/";

lazy_static! {
  pub static ref CONFIG: Config = Config::new().unwrap();
}

pub struct Config {
  pub metadata: Metadata,
  pub settings: Settings,
  pub offset: Offset,
}

pub struct MainConfig {
  pub metadata: Metadata,
  pub settings: Settings,
}

pub struct Metadata {
  pub name: String,
}

pub struct Settings {
  pub log_level: i64,
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
  pub version: String,
  pub checksum: u32,
  pub string_copy: usize,
  pub string_copy_n: usize,
  pub string_append: usize,
  pub string_append_0: usize,
  pub string_append_n: usize,
  pub convert_ulong_to_string: usize,
  pub addst: usize,
  pub addst_top: usize,
  pub addcoloredst: usize,
  pub addst_flag: usize,
  pub standardstringentry: usize,
  pub simplify_string: usize,
  pub upper_case_string: usize,
  pub lower_case_string: usize,
  pub capitalize_string_words: usize,
  pub capitalize_string_first_word: usize,
  pub addchar: usize,
  pub addchar_top: usize,
  pub add_texture: usize,
  pub gps_allocate: usize,
  pub cleanup_arrays: usize,
  pub screen_to_texid: usize,
  pub screen_to_texid_top: usize,
  pub loading_world_new_game_loop: usize,
  pub loading_world_continuing_game_loop: usize,
  pub loading_world_start_new_game_loop: usize,
  pub menu_interface_loop: usize,
  pub keybinding: u32,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let pe_timestamp = Self::pe_timestamp(Path::new(EXE_FILE))?;
    let main_config = Self::parse_config(Path::new(CONFIG_FILE))?;
    match Self::walk_offsets(Path::new(OFFSETS_DIR), pe_timestamp) {
      Ok(offsets) => Ok(Self {
        metadata: main_config.metadata,
        settings: main_config.settings,
        offset: offsets,
      }),
      Err(_) => Err("Config Error".into()),
    }
  }

  fn read_toml(filename: &Path) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let result = content.parse::<Table>()?;
    Ok(result)
  }

  fn pe_timestamp(path: &Path) -> Result<u32, Box<dyn Error>> {
    let pefile = VecPE::from_disk_file(path)?;
    Ok(pefile.get_nt_headers_64()?.file_header.time_date_stamp)
  }

  fn walk_offsets(path: &Path, target_timestamp: u32) -> Result<Offset, Box<dyn Error>> {
    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
      let entry = entry.unwrap();
      let pentry = entry.path();
      if !pentry.is_file() {
        continue;
      }
      let offsets = Self::parse_offsets(pentry)?;
      if offsets.checksum == target_timestamp {
        return Ok(offsets);
      }
    }

    unsafe {
      utils::message_box(
        format!(
          "unable to find offsets file for current version of DF\nchecksum: 0x{:x}",
          target_timestamp
        )
        .as_str(),
        "dfint hook error",
        utils::MessageIconType::Error,
      );
    }
    std::process::exit(2);
    // Err("Unable to find offsets file".into())
  }

  fn parse_config(path: &Path) -> Result<MainConfig, Box<dyn Error>> {
    let root = Self::read_toml(path)?;
    let metadata = root["metadata"].as_table().unwrap();
    let settings = root["settings"].as_table().unwrap();
    Ok(MainConfig {
      metadata: Metadata {
        name: String::from(metadata["name"].as_str().unwrap()),
      },
      settings: Settings {
        log_level: settings["log_level"].as_integer().unwrap(),
        log_file: String::from(settings["log_file"].as_str().unwrap()),
        crash_report: settings["crash_report"].as_bool().unwrap(),
        crash_report_dir: String::from(settings["crash_report_dir"].as_str().unwrap()),
        enable_search: settings["enable_search"].as_bool().unwrap(),
        enable_translation: settings["enable_translation"].as_bool().unwrap(),
        enable_patches: settings["enable_patches"].as_bool().unwrap(),
        dictionary: String::from(settings["dictionary"].as_str().unwrap()),
        watchdog: settings["watchdog"].as_bool().unwrap(),
      },
    })
  }

  fn parse_offsets(path: &Path) -> Result<Offset, Box<dyn Error>> {
    let root = Self::read_toml(path)?;
    let metadata = root["metadata"].as_table().unwrap();
    let offsets = root["offsets"].as_table().unwrap();
    Ok(Offset {
      version: String::from(metadata["version"].as_str().unwrap_or("none")),
      checksum: u32::try_from(metadata["checksum"].as_integer().unwrap())?,
      string_copy: usize::try_from(offsets["string_copy"].as_integer().unwrap())?,
      string_copy_n: usize::try_from(offsets["string_copy_n"].as_integer().unwrap())?,
      string_append: usize::try_from(offsets["string_append"].as_integer().unwrap())?,
      string_append_0: usize::try_from(offsets["string_append_0"].as_integer().unwrap())?,
      string_append_n: usize::try_from(offsets["string_append_n"].as_integer().unwrap())?,
      convert_ulong_to_string: usize::try_from(
        offsets["convert_ulong_to_string"].as_integer().unwrap(),
      )?,
      addst: usize::try_from(offsets["addst"].as_integer().unwrap())?,
      addst_top: usize::try_from(offsets["addst_top"].as_integer().unwrap())?,
      addcoloredst: usize::try_from(offsets["addcoloredst"].as_integer().unwrap())?,
      addst_flag: usize::try_from(offsets["addst_flag"].as_integer().unwrap())?,
      standardstringentry: usize::try_from(offsets["standardstringentry"].as_integer().unwrap())?,
      simplify_string: usize::try_from(offsets["simplify_string"].as_integer().unwrap())?,
      upper_case_string: usize::try_from(offsets["upper_case_string"].as_integer().unwrap())?,
      lower_case_string: usize::try_from(offsets["lower_case_string"].as_integer().unwrap())?,
      capitalize_string_words: usize::try_from(
        offsets["capitalize_string_words"].as_integer().unwrap(),
      )?,
      capitalize_string_first_word: usize::try_from(
        offsets["capitalize_string_first_word"].as_integer().unwrap(),
      )?,
      addchar: usize::try_from(offsets["addchar"].as_integer().unwrap())?,
      addchar_top: usize::try_from(offsets["addchar_top"].as_integer().unwrap())?,
      add_texture: usize::try_from(offsets["add_texture"].as_integer().unwrap())?,
      gps_allocate: usize::try_from(offsets["gps_allocate"].as_integer().unwrap())?,
      cleanup_arrays: usize::try_from(offsets["cleanup_arrays"].as_integer().unwrap())?,
      screen_to_texid: usize::try_from(offsets["screen_to_texid"].as_integer().unwrap())?,
      screen_to_texid_top: usize::try_from(offsets["screen_to_texid_top"].as_integer().unwrap())?,
      loading_world_new_game_loop: usize::try_from(
        offsets["loading_world_new_game_loop"].as_integer().unwrap(),
      )?,
      loading_world_continuing_game_loop: usize::try_from(
        offsets["loading_world_continuing_game_loop"].as_integer().unwrap(),
      )?,
      loading_world_start_new_game_loop: usize::try_from(
        offsets["loading_world_start_new_game_loop"].as_integer().unwrap(),
      )?,
      menu_interface_loop: usize::try_from(offsets["menu_interface_loop"].as_integer().unwrap())?,
      keybinding: u32::try_from(match offsets.get("keybinding") {
        Some(value) => value.as_integer().unwrap(),
        None => 0,
      })?,
    })
  }
}

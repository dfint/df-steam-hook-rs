use std::error::Error;
use std::path::Path;

use crate::utils;
use exe::{VecPE, PE};
use static_init::dynamic;
use walkdir::WalkDir;

static EXE_FILE: &str = "./Dwarf Fortress.exe";
static CONFIG_FILE: &str = "./dfint_data/dfint_config.toml";
static OFFSETS_DIR: &str = "./dfint_data/offsets/";

#[dynamic]
pub static CONFIG: Config = Config::new().unwrap();

// lazy_static! {
//   pub static ref CONFIG: Config = Config::new().unwrap();
// }

pub struct Config {
  pub metadata: ConfigMetadata,
  pub settings: Settings,
  pub offset: OffsetsValues,
  pub offset_metadata: OffsetsMetadata,
  pub hook_version: String,
}

#[derive(Deserialize)]
pub struct MainConfig {
  pub metadata: ConfigMetadata,
  pub settings: Settings,
}

#[derive(Deserialize)]
pub struct ConfigMetadata {
  pub name: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Offsets {
  pub metadata: OffsetsMetadata,
  pub offsets: OffsetsValues,
}

#[derive(Deserialize)]
pub struct OffsetsMetadata {
  pub name: String,
  pub version: String,
  pub checksum: u32,
}

#[derive(Deserialize)]
pub struct OffsetsValues {
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
  pub addst_template: Option<usize>,
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
  pub keybinding: Option<u32>,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let pe_timestamp = Self::pe_timestamp(Path::new(EXE_FILE))?;
    let main_config = Self::parse_toml::<MainConfig>(Path::new(CONFIG_FILE))?;
    match Self::walk_offsets(Path::new(OFFSETS_DIR), pe_timestamp) {
      Ok(offsets) => Ok(Self {
        metadata: main_config.metadata,
        settings: main_config.settings,
        offset: offsets.offsets,
        offset_metadata: offsets.metadata,
        hook_version: match option_env!("HOOK_VERSION") {
          Some(version) => String::from(version),
          None => String::from("not-defined"),
        },
      }),
      Err(_) => Err("Config Error".into()),
    }
  }

  fn pe_timestamp(path: &Path) -> Result<u32, Box<dyn Error>> {
    let pefile = VecPE::from_disk_file(path)?;
    Ok(pefile.get_nt_headers_64()?.file_header.time_date_stamp)
  }

  fn walk_offsets(path: &Path, target_timestamp: u32) -> Result<Offsets, Box<dyn Error>> {
    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
      let entry = entry.unwrap();
      let pentry = entry.path();
      if !pentry.is_file() {
        continue;
      }
      let offsets = Self::parse_toml::<Offsets>(pentry)?;
      if offsets.metadata.checksum == target_timestamp {
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

  fn parse_toml<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let data: T = toml::from_str(content.as_str())?;
    Ok(data)
  }
}

use anyhow::{anyhow, Result};
use std::path::Path;

use crate::utils;

#[cfg(target_os = "windows")]
const EXE_FILE: &'static str = "./Dwarf Fortress.exe";

#[cfg(target_os = "linux")]
const EXE_FILE: &'static str = "./dwarfort";

const CONFIG_FILE: &'static str = "./dfint_data/dfint_config.toml";
const OFFSETS_DIR: &'static str = "./dfint_data/offsets/";

#[static_init::dynamic]
pub static CONFIG: Config = Config::new().unwrap();

pub struct Config {
  pub metadata: ConfigMetadata,
  pub settings: Settings,
  pub offset: OffsetsValues,
  pub offset_metadata: OffsetsMetadata,
  pub symbol: Option<SymbolsValues>,
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
  pub log_level: usize,
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
  pub symbols: Option<SymbolsValues>,
}

#[derive(Deserialize)]
pub struct OffsetsMetadata {
  pub name: String,
  pub version: String,
  pub checksum: u32,
}

#[derive(Deserialize)]
pub struct OffsetsValues {
  pub string_copy_n: Option<usize>,
  pub string_append_n: Option<usize>,
  pub addst: Option<usize>,
  pub addst_top: Option<usize>,
  pub addst_flag: Option<usize>,
  pub standardstringentry: Option<usize>,
  pub simplify_string: Option<usize>,
  pub upper_case_string: Option<usize>,
  pub lower_case_string: Option<usize>,
  pub capitalize_string_words: Option<usize>,
  pub capitalize_string_first_word: Option<usize>,
  pub utf_input: Option<usize>,
}

#[derive(Deserialize)]
pub struct SymbolsValues {
  pub addst: Option<Vec<String>>,
  pub addst_top: Option<Vec<String>>,
  pub addst_flag: Option<Vec<String>>,
  pub standardstringentry: Option<Vec<String>>,
  pub simplify_string: Option<Vec<String>>,
  pub upper_case_string: Option<Vec<String>>,
  pub lower_case_string: Option<Vec<String>>,
  pub capitalize_string_words: Option<Vec<String>>,
  pub capitalize_string_first_word: Option<Vec<String>>,
  pub std_string_append: Option<Vec<String>>,
  pub std_string_assign: Option<Vec<String>>,
  pub enabler: Option<Vec<String>>,
}

impl Config {
  pub fn new() -> Result<Self> {
    let checksum = Self::checksum(Path::new(EXE_FILE))?;
    let main_config = Self::parse_toml::<MainConfig>(Path::new(CONFIG_FILE))?;
    match Self::walk_offsets(Path::new(OFFSETS_DIR), checksum) {
      Ok(offsets) => Ok(Self {
        metadata: main_config.metadata,
        settings: main_config.settings,
        offset: offsets.offsets,
        offset_metadata: offsets.metadata,
        symbol: offsets.symbols,
        hook_version: match option_env!("HOOK_VERSION") {
          Some(version) => String::from(version),
          None => String::from("not-defined"),
        },
      }),
      Err(e) => Err(anyhow!("Config error {:?}", e)),
    }
  }

  #[cfg(target_os = "windows")]
  fn checksum(path: &Path) -> Result<u32> {
    use exe::{VecPE, PE};
    let pefile = VecPE::from_disk_file(path)?;
    Ok(pefile.get_nt_headers_64()?.file_header.time_date_stamp)
  }

  #[cfg(target_os = "linux")]
  fn checksum(path: &Path) -> Result<u32> {
    let mut crc = checksum::crc::Crc::new(path.to_str().unwrap());
    match crc.checksum() {
      Ok(checksum) => Ok(checksum.crc32),
      Err(e) => Err(anyhow!("Checksum error {:?}", e).into()),
    }
  }

  fn walk_offsets(path: &Path, target_checksum: u32) -> Result<Offsets> {
    for entry in std::fs::read_dir(path)? {
      let entry = entry?;
      let pentry = entry.path();
      if !pentry.is_file() {
        continue;
      }
      let offsets = Self::parse_toml::<Offsets>(&pentry)?;
      if offsets.metadata.checksum == target_checksum {
        return Ok(offsets);
      }
    }

    utils::message_box(
      format!(
        "unable to find offsets file for current version of DF\nchecksum: 0x{:x}",
        target_checksum
      )
      .as_str(),
      "dfint hook error",
      utils::MessageIconType::Error,
    );

    std::process::exit(2);
  }

  fn parse_toml<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path)?;
    let data: T = toml::from_str(content.as_str())?;
    Ok(data)
  }
}

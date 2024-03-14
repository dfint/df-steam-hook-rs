#[allow(unused_imports)]
use anyhow::{anyhow, Context, Result};
use std::path::Path;

use crate::{
  constants::{PATH_CONFIG, PATH_EXE, PATH_OFFSETS},
  encoding::Encoding,
  utils,
};

#[static_init::dynamic]
pub static CONFIG: Config = Config::new();

pub struct Config {
  pub metadata: ConfigMetadata,
  pub settings: Settings,
  pub offset_metadata: OffsetsMetadata,
  pub offset: Option<OffsetsValues>,
  pub symbol: Option<SymbolsValues>,
  pub hook_version: String,
  pub encoding: Encoding,
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
  pub enable_search: bool,
  pub enable_translation: bool,
  pub watchdog: bool,
}

#[derive(Deserialize)]
pub struct Offsets {
  pub metadata: OffsetsMetadata,
  pub offsets: Option<OffsetsValues>,
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
  pub std_string_ctor: Option<usize>,
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
  pub fn new() -> Self {
    let checksum = Self::checksum(PATH_EXE).unwrap_or(0);
    let main_config = Self::parse_toml::<MainConfig>(PATH_CONFIG).unwrap();
    let encoding = Encoding::new();
    let hook_version = match option_env!("HOOK_VERSION") {
      Some(version) => String::from(version),
      None => String::from("not-defined"),
    };

    let (offset_metadata, offset, symbol) = match Self::parse_toml::<Offsets>(PATH_OFFSETS) {
      Ok(o) if o.metadata.checksum == checksum => (o.metadata, o.offsets, o.symbols),
      _ => {
        utils::message_box(
          "dfint hook error",
          format!("This DF version is not supported.\nDF checksum: 0x{:x}", checksum).as_str(),
          utils::MessageIconType::Error,
        );
        (
          OffsetsMetadata {
            name: String::from("not found"),
            version: String::from("not found"),
            checksum,
          },
          None,
          None,
        )
      }
    };

    Self {
      metadata: main_config.metadata,
      settings: main_config.settings,
      offset_metadata,
      offset,
      symbol,
      hook_version,
      encoding,
    }
  }

  #[cfg(target_os = "windows")]
  fn checksum(path: &str) -> Result<u32> {
    use exe::{VecPE, PE};
    let pefile = VecPE::from_disk_file(Path::new(path))?;
    Ok(pefile.get_nt_headers_64()?.file_header.time_date_stamp)
  }

  #[cfg(target_os = "linux")]
  fn checksum(path: &str) -> Result<u32> {
    let mut crc = checksum::crc::Crc::new(path);
    match crc.checksum() {
      Ok(checksum) => Ok(checksum.crc32),
      Err(e) => Err(anyhow!("Checksum error {:?}", e).into()),
    }
  }

  fn parse_toml<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Result<T> {
    let content = std::fs::read_to_string(Path::new(path))?;
    let data: T = toml::from_str(content.as_str())?;
    Ok(data)
  }
}

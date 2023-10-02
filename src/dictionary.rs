use std::collections::HashMap;
use std::io::prelude::*;

use crate::config::CONFIG;
use crate::utils;

#[static_init::dynamic]
pub static DICTIONARY: Dictionary = Dictionary::new(&CONFIG.settings.dictionary);

#[allow(dead_code)]
pub struct Dictionary {
  map: HashMap<String, Vec<u8>>,
  path: &'static str,
}

impl Dictionary {
  pub fn new(path: &'static String) -> Self {
    Self {
      map: Dictionary::load(path).unwrap_or_else(|_| {
        log::error!("unable to load dictionary {}", path);
        utils::message_box(
          "dfint hook error",
          format!("Unable to load dictionary {}", path).as_str(),
          utils::MessageIconType::Warning,
        );
        HashMap::<String, Vec<u8>>::new()
      }),
      path,
    }
  }

  pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
    self.map.get(key)
  }

  pub fn _size(&self) -> usize {
    self.map.capacity()
  }

  pub fn _data(&self) -> &HashMap<String, Vec<u8>> {
    &self.map
  }

  pub fn _reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.map = Self::load(self.path)?;
    Ok(())
  }

  #[allow(unused_must_use)]
  fn load(path: &str) -> Result<HashMap<String, Vec<u8>>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents);
    let mut map = HashMap::<String, Vec<u8>>::new();
    for item in regex::bytes::Regex::new(r#"(?-u)"(.+)","(.+)""#)?.captures_iter(&contents) {
      let mut v = Vec::<u8>::from(&item[2]);
      v.push(0);
      map.insert(String::from_utf8_lossy(&item[1]).to_string(), v);
    }
    Ok(map)
  }
}

use anyhow::Result;
use std::collections::HashMap;
use std::io::prelude::*;

use crate::constants::PATH_DICTIONARY;
use crate::utils;

#[static_init::dynamic]
pub static DICTIONARY: Dictionary = Dictionary::new(PATH_DICTIONARY);

#[allow(dead_code)]
pub struct Dictionary {
  map: HashMap<Vec<u8>, Vec<u8>>,
  path: &'static str,
}

impl Dictionary {
  pub fn new(path: &'static str) -> Self {
    Self {
      map: match Dictionary::load(path) {
        Ok(value) => value,
        Err(_) => {
          log::error!("unable to load dictionary {path}");
          utils::message_box(
            "dfint hook error",
            format!("Unable to load dictionary {path}").as_str(),
            utils::MessageIconType::Warning,
          );
          HashMap::<Vec<u8>, Vec<u8>>::new()
        }
      },
      path,
    }
  }

  pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
    self.map.get(key)
  }

  pub fn size(&self) -> usize {
    self.map.len()
  }

  pub fn _data(&self) -> &HashMap<Vec<u8>, Vec<u8>> {
    &self.map
  }

  pub fn _reload(&mut self) -> Result<()> {
    self.map = Self::load(self.path)?;
    Ok(())
  }

  #[allow(unused_must_use)]
  fn load(path: &str) -> Result<HashMap<Vec<u8>, Vec<u8>>> {
    let mut file = std::fs::File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents);
    let mut map = HashMap::<Vec<u8>, Vec<u8>>::new();
    let quote = &b"\""[0];
    for item in regex::bytes::Regex::new(r#"(?-u)"(.+)","(.+)""#)?.captures_iter(&contents) {
      let mut k = item[1].to_vec();
      let mut v = item[2].to_vec();
      v.push(0);
      k.dedup_by(|a, b| a == quote && b == quote);
      v.dedup_by(|a, b| a == quote && b == quote);
      map.insert(k, v);
    }
    Ok(map)
  }
}

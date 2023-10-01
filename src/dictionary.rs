use std::collections::HashMap;
use std::io::prelude::*;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;

use crate::config::CONFIG;

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
      map: Dictionary::load(path).unwrap(),
      path,
    }
  }

  pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
    self.map.get(key)
  }

  pub fn _size(&self) -> usize {
    self.map.capacity()
  }

  pub fn _reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.map = Self::load(self.path)?;
    Ok(())
  }

  fn load(path: &str) -> Result<HashMap<String, Vec<u8>>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut reader = DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1251)).build(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut map = HashMap::<String, Vec<u8>>::new();
    for item in Regex::new(r#""(.+)","(.+)""#)?.captures_iter(&contents) {
      let mut v = Vec::from(WINDOWS_1251.encode(item.get(2).unwrap().as_str()).0.as_ref());
      v.push(0);
      map.insert(String::from(item.get(1).unwrap().as_str()), v);
    }
    Ok(map)
  }
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;

use crate::config::CONFIG;

lazy_static! {
  pub static ref DICTIONARY: Dictionary = Dictionary::new(&CONFIG.settings.dictionary);
}

pub struct Dictionary {
  map: HashMap<String, Vec<u8>>,
}

impl Dictionary {
  pub fn new(path: &String) -> Self {
    Self {
      map: Dictionary::load(path).unwrap(),
    }
  }

  pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
    self.map.get(key)
  }

  pub fn _size(&self) -> usize {
    self.map.capacity()
  }

  fn load(path: &str) -> Result<HashMap<String, Vec<u8>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = DecodeReaderBytesBuilder::new()
      .encoding(Some(WINDOWS_1251))
      .build(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut map = HashMap::<String, Vec<u8>>::new();

    for item in Regex::new(r#""(.+)","(.+)""#)?.captures_iter(&contents) {
      map.insert(
        String::from(item.get(1).unwrap().as_str()),
        Vec::from(
          WINDOWS_1251
            .encode(item.get(2).unwrap().as_str())
            .0
            .as_ref(),
        ),
      );
    }

    Ok(map)
  }
}

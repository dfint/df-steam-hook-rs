use anyhow::{anyhow, Result};
use std::{collections::HashMap, path::Path};
use toml::{map::Map, Table, Value};

use crate::constants::PATH_ENCODING;

pub struct Encoding {
  pub capitalize: Vec<u8>,
  pub lowercast: Vec<u8>,
  pub simplify: Vec<u8>,
  pub uppercase: Vec<u8>,
  pub lowercase: Vec<u8>,
  pub utf: HashMap<u32, u8>,
  pub parsed: bool,
}

impl Encoding {
  pub fn new() -> Self {
    match Self::parse_encodings(Path::new(PATH_ENCODING)) {
      Ok(v) => v,
      Err(_) => Self::default(),
    }
  }

  fn parse_encodings(path: &Path) -> Result<Encoding> {
    let content = std::fs::read_to_string(path)?;
    let data = content.parse::<Table>()?;

    let capitalize_table = data["maps"]["capitalize"].as_table().ok_or(anyhow!("parsing err"))?;
    let lowercast_table = data["maps"]["lowercast"].as_table().ok_or(anyhow!("parsing err"))?;
    let simplify_table = data["maps"]["simplify"].as_table().ok_or(anyhow!("parsing err"))?;
    let uppercase_table = data["maps"]["uppercase"].as_table().ok_or(anyhow!("parsing err"))?;
    let lowercase_table = data["maps"]["lowercase"].as_table().ok_or(anyhow!("parsing err"))?;
    let utf_table = data["maps"]["utf"].as_table().ok_or(anyhow!("parsing err"))?;

    let capitalize = Self::shift_transition(capitalize_table, None)?;
    let lowercast = Self::shift_transition(lowercast_table, None)?;

    Ok(Encoding {
      capitalize: capitalize.clone(),
      lowercast: lowercast.clone(),
      simplify: Self::replace_transition(simplify_table, Some(lowercast.clone()))?,
      uppercase: Self::replace_transition(uppercase_table, Some(capitalize))?,
      lowercase: Self::replace_transition(lowercase_table, Some(lowercast))?,
      utf: Self::utf_transition(utf_table)?,
      parsed: true,
    })
  }

  fn utf_transition(map: &Map<String, Value>) -> Result<HashMap<u32, u8>> {
    let mut out: HashMap<u32, u8> = HashMap::new();
    for (k, v) in map {
      out.insert(k.parse::<u32>()?, v.as_integer().ok_or(anyhow!("parsing err"))? as u8);
    }
    Ok(out)
  }

  fn shift_transition(map: &Map<String, Value>, base: Option<Vec<u8>>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = match base {
      Some(b) => b,
      None => (0..=255).collect(),
    };
    for (k, v) in map {
      for i in Self::str_to_array(k)? {
        out[i as usize] = out[i as usize] + v.as_integer().ok_or(anyhow!("parsing err"))? as u8;
      }
    }
    Ok(out)
  }

  fn replace_transition(map: &Map<String, Value>, base: Option<Vec<u8>>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = match base {
      Some(b) => b,
      None => (0..=255).collect(),
    };
    for (k, v) in map {
      for i in Self::str_to_array(k)? {
        out[i as usize] = v.as_integer().ok_or(anyhow!("parsing err"))? as u8;
      }
    }
    Ok(out)
  }

  fn str_to_array(value: &String) -> Result<Vec<u8>> {
    if value.contains(":") {
      let r: Vec<&str> = value.split(":").collect();
      let start = r[0].parse::<u8>()?;
      let end = r[1].parse::<u8>()?;
      Ok((start..=end).collect())
    } else if value.contains("|") {
      let mut out: Vec<u8> = vec![];
      for v in value.split("|") {
        out.push(v.parse::<u8>()?);
      }
      Ok(out)
    } else {
      Ok(vec![value.parse::<u8>()?])
    }
  }
}

impl Default for Encoding {
  fn default() -> Self {
    let blank: [u8; 256] = core::array::from_fn(|i| i as u8);
    Self {
      capitalize: blank.clone().to_vec(),
      lowercast: blank.clone().to_vec(),
      simplify: blank.clone().to_vec(),
      uppercase: blank.clone().to_vec(),
      lowercase: blank.to_vec(),
      utf: HashMap::new(),
      parsed: false,
    }
  }
}

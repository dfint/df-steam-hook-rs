extern crate proc_macro;

use proc_macro::TokenStream;
use regex::Regex;

#[proc_macro_attribute]
pub fn attach(attr: TokenStream, input: TokenStream) -> TokenStream {
  let parsed = ParseFn::new(&input.clone().to_string().as_str());
  let attach = format!(
    "pub unsafe fn attach_{}() -> Result<(), Box<dyn Error>> {{ let target = mem::transmute(utils::address(CONFIG.offset.{})); handle_{}.initialize(target, {})?.enable()?; Ok(()) }}",
    parsed.name, parsed.name, parsed.name, parsed.name
  );
  let mut return_type = String::from("");
  if parsed.return_type != "" {
    return_type = format!(" -> {}", parsed.return_type);
  }
  let static_detour = format!(
    "static_detour! {{ static handle_{}: unsafe extern \"{}\" fn({}){}; }}",
    parsed.name,
    attr,
    parsed.arg_type.join(", "),
    return_type
  );
  let result = format!(
    "{}\n{}\n{}",
    static_detour.as_str(),
    input
      .to_string()
      .replace("original!", format!("handle_{}.call", parsed.name).as_str()),
    attach.as_str()
  );
  // println!("{:}", result);
  result.parse().unwrap()
}

#[allow(dead_code)]
struct ParseFn {
  name: String,
  sig: String,
  return_type: String,
  arg_type: Vec<String>,
  arg_name: Vec<String>,
}

impl ParseFn {
  pub fn new(value: &str) -> Self {
    let s = String::from(value.clone()).replace("\n", " ");
    let name = Regex::new(r"fn\s([\w]+)\(")
      .unwrap()
      .captures(value)
      .unwrap()
      .get(1)
      .unwrap()
      .as_str();
    let sig = match Regex::new(r"fn\s*[\w]+\((.*?)\)").unwrap().captures(s.as_str()) {
      Some(item) => item.get(1).unwrap().as_str(),
      None => "",
    };
    let return_type = match Regex::new(r"->\s(.*?)\s\{").unwrap().captures(s.as_str()) {
      Some(item) => item.get(1).unwrap().as_str(),
      None => "",
    };
    let mut arg_type = Vec::<String>::new();
    for item in Regex::new(r"[\w]+\s:\s([\w\*\s&']+)").unwrap().captures_iter(sig) {
      arg_type.push(String::from(item.get(1).unwrap().as_str()));
    }
    let mut arg_name = Vec::<String>::new();
    for item in Regex::new(r"([\w]+)\s:\s[\w\*\s&]+").unwrap().captures_iter(sig) {
      arg_name.push(String::from(item.get(1).unwrap().as_str()));
    }

    Self {
      name: String::from(name),
      sig: String::from(sig),
      return_type: String::from(return_type),
      arg_type,
      arg_name,
    }
  }
}

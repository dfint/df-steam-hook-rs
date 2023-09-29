#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

struct Args {
  offset: usize,
  module: String,
  symbol: String,
  bypass: bool,
  without_base: bool,
}

impl Args {
  pub fn parse(args: TokenStream) -> Self {
    let mut offset = 0;
    let mut module = String::from("");
    let mut symbol = String::from("");
    let mut bypass = false;
    let mut without_base = false;
    for arg_pair in args.to_string().split(",") {
      let arg = arg_pair.split("=").collect::<Vec<&str>>();
      match arg[0].trim() {
        "offset" => offset = usize::from_str_radix(&arg[1].trim().replace("\"", ""), 16).unwrap(),
        "module" => module = String::from(arg[1].trim()),
        "symbol" => symbol = String::from(arg[1].trim()),
        "bypass" => bypass = true,
        "without_base" => without_base = true,
        _ => (),
      }
    }
    Self {
      offset,
      module,
      symbol,
      bypass,
      without_base,
    }
  }
}

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = Args::parse(args);
  let item: syn::Item = syn::parse_macro_input!(input);
  if let syn::Item::Fn(function) = item {
    let syn::ItemFn {
      block,
      vis,
      sig:
        syn::Signature {
          ident,
          unsafety,
          constness,
          abi,
          output,
          inputs,
          ..
        },
      ..
    } = function;

    let attach_ident = format_ident!("attach_{}", ident);
    let detach_ident = format_ident!("detach_{}", ident);
    let handle_ident = format_ident!("handle_{}", ident);
    let ret_type = quote!(#output).to_string();
    let inputs_unnamed = quote!(#inputs)
      .to_string()
      .split(",")
      .map(|arg| arg.split(":").collect::<Vec<&str>>()[1])
      .collect::<Vec<&str>>()
      .join(",");

    let mut attach = quote!(
      pub unsafe fn #attach_ident() -> Result<(), Box<dyn std::error::Error>> {
        let target = target();
        #handle_ident.initialize(target, #ident)?.enable()?;
        Ok(())
      }
    )
    .to_string();
    if args.offset > 0 {
      attach = attach.replace(
        "target()",
        format!("std::mem::transmute(utils::address({}))", args.offset).as_str(),
      );
    } else if args.module != "" && args.symbol != "" {
      attach = attach.replace(
        "target()",
        format!(
          "utils::symbol_handle::<fn({}) {}>({}, {})",
          inputs_unnamed, ret_type, args.module, args.symbol
        )
        .as_str(),
      );
    } else if args.without_base {
      attach = attach.replace(
        "target()",
        format!("std::mem::transmute(CONFIG.offset.{})", ident.to_string()).as_str(),
      );
    } else {
      attach = attach.replace(
        "target()",
        format!(
          "std::mem::transmute(utils::address(CONFIG.offset.{}))",
          ident.to_string()
        )
        .as_str(),
      );
    }

    if args.bypass {
      attach = quote!(
        pub unsafe fn #attach_ident() -> Result<(), Box<dyn std::error::Error>> {
          Ok(())
        }
      )
      .to_string();
    }

    let result = quote!(
      pub unsafe fn #detach_ident() -> Result<(), Box<dyn std::error::Error>> {
        #handle_ident.disable()?;
        Ok(())
      }
      static_detour! { static #handle_ident: unsafe #abi fn() #output; }
      #vis #unsafety #constness fn #ident(#inputs) #output #block
    );

    return format!(
      "{}\n{}",
      attach.to_string(),
      result
        .to_string()
        .replace("original!", format!("handle_{}.call", ident.to_string()).as_str())
        .replace("fn()", format!("fn({})", inputs_unnamed).as_str())
    )
    .parse()
    .unwrap();
  } else {
    panic!("Fatal error in hook macro")
  }
}

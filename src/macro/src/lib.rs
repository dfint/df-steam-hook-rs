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
  by_offset: bool,
  by_symbol: bool,
}

impl Args {
  pub fn parse(args: TokenStream) -> Self {
    let mut offset = 0;
    let mut module = String::from("");
    let mut symbol = String::from("");
    let mut bypass = false;
    let mut by_offset: bool = false;
    let mut by_symbol: bool = false;
    for arg_pair in args.to_string().split(",") {
      let arg = arg_pair.split("=").collect::<Vec<&str>>();
      match arg[0].trim() {
        "offset" => offset = usize::from_str_radix(&arg[1].trim().replace("\"", ""), 16).unwrap(),
        "module" => module = String::from(arg[1].trim()),
        "symbol" => symbol = String::from(arg[1].trim()),
        "bypass" => bypass = true,
        "by_offset" => by_offset = true,
        "by_symbol" => by_symbol = true,
        _ => (),
      }
    }
    Self {
      offset,
      module,
      symbol,
      bypass,
      by_offset,
      by_symbol,
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
    let handle_ident = format_ident!("handle_{}", ident);
    let enable_ident = format_ident!("enable_{}", ident);
    let disable_ident = format_ident!("disable_{}", ident);
    let ret_type = quote!(#output).to_string();
    let inputs_unnamed = quote!(#inputs)
      .to_string()
      .split(",")
      .map(|arg| arg.split(":").collect::<Vec<&str>>()[1])
      .collect::<Vec<&str>>()
      .join(",");

    let mut attach = quote!(
      pub unsafe fn #attach_ident() -> Result<()> {
        let target = target();
        #handle_ident.initialize(target, #ident)?;
        #enable_ident()?;
        Ok(())
      }

      pub unsafe fn #enable_ident() -> Result<()> {
        #handle_ident.enable()?;
        Ok(())
      }

      pub unsafe fn #disable_ident() -> Result<()> {
        #handle_ident.disable()?;
        Ok(())
      }
    )
    .to_string();

    attach = match (args.offset, args.module, args.symbol, args.by_offset, args.by_symbol) {
      (o, _, _, _, _) if o > 0 => attach.replace(
        "target()",
        format!("std::mem::transmute(utils::address({}))", o).as_str(),
      ),
      (_, m, s, _, _) if m == "self" && s != "" => attach.replace(
        "target()",
        format!(
          "utils::symbol_handle_self::<fn({}) {}>({})",
          inputs_unnamed, ret_type, s
        )
        .as_str(),
      ),
      (_, m, s, _, _) if m != "" && s != "" => attach.replace(
        "target()",
        format!(
          "utils::symbol_handle::<fn({}) {}>({}, {})",
          inputs_unnamed, ret_type, m, s
        )
        .as_str(),
      ),
      (_, _, _, _, bs) if bs => attach.replace(
        "target()",
        format!(
          "utils::symbol_handle::<fn({}) {}>(&CONFIG.symbol.as_ref().unwrap().{}.as_ref().unwrap()[0], &CONFIG.symbol.as_ref().unwrap().{}.as_ref().unwrap()[1])",
          inputs_unnamed, ret_type, ident.to_string(), ident.to_string()
        )
        .as_str(),
      ),
      (_, _, _, bo, _) if bo => attach.replace(
        "target()",
        format!(
          "std::mem::transmute(utils::address(CONFIG.offset.{}.unwrap()))",
          ident.to_string()
        )
        .as_str(),
      ),
      (_, _, _, _, _) => attach.replace(
        "target()",
        format!(
          "std::mem::transmute(utils::address(CONFIG.offset.{}.unwrap()))",
          ident.to_string()
        )
        .as_str(),
      ),
    };

    if args.bypass {
      attach = quote!(
        pub unsafe fn #attach_ident() -> Result<()> {
          Ok(())
        }

        pub unsafe fn #enable_ident() -> Result<()> {
          Ok(())
        }

        pub unsafe fn #disable_ident() -> Result<()> {
          Ok(())
        }
      )
      .to_string();
    }

    let result = quote!(
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

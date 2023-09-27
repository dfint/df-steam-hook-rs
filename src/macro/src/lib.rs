#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

struct Args {
  offset: usize,
  module: String,
  symbol: String,
}

impl Args {
  pub fn parse(args: TokenStream) -> Self {
    let mut offset = 0;
    let mut module = String::from("");
    let mut symbol = String::from("");
    for arg_pair in args.to_string().split(",") {
      let arg = arg_pair.split("=").collect::<Vec<&str>>();
      match arg[0].trim() {
        "offset" => offset = arg[1].trim().replace("\"", "").parse::<usize>().unwrap(),
        "module" => module = String::from(arg[1].trim()),
        "symbol" => symbol = String::from(arg[1].trim()),
        _ => (),
      }
    }
    Self { offset, module, symbol }
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

    let mut attach = quote!(
      pub unsafe fn #attach_ident() -> Result<(), Box<dyn Error>> {
        let target = target();
        #handle_ident.initialize(target, #ident)?.enable()?;
        Ok(())
      }
    )
    .to_string();
    if args.offset > 0 {
      attach = attach.replace(
        "target()",
        format!("mem::transmute(utils::address({}))", args.offset).as_str(),
      );
    } else if args.module != "" && args.symbol != "" {
      attach = attach.replace(
        "target()",
        format!(
          "mem::transmute(utils::symbol_handle::<fn()>({}, {}))",
          args.module, args.symbol
        )
        .as_str(),
      );
    } else {
      attach = attach.replace(
        "target()",
        format!("mem::transmute(utils::address(CONFIG.offset.{}))", ident.to_string()).as_str(),
      );
    }

    let output = quote!(
      pub unsafe fn #detach_ident() -> Result<(), Box<dyn Error>> {
        #handle_ident.disable()?;
        Ok(())
      }
      static_detour! { static #handle_ident: unsafe #abi fn() #output; }
      #vis #unsafety #constness fn #ident(#inputs) #output #block
    );

    let inputs_unnamed = format!("{}", quote!(#inputs))
      .split(",")
      .map(|arg| arg.split(":").collect::<Vec<&str>>()[1])
      .collect::<Vec<&str>>()
      .join(",");

    return format!(
      "{}\n{}",
      attach.to_string(),
      output
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

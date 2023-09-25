#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn hook(_attr: TokenStream, input: TokenStream) -> TokenStream {
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

    let output = quote!(
      pub unsafe fn #attach_ident() -> Result<(), Box<dyn Error>> {
        let target = mem::transmute(utils::address(CONFIG.offset.#ident));
        #handle_ident.initialize(target, #ident)?.enable()?;
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

    return output
      .to_string()
      .replace("original!", format!("handle_{}.call", ident.to_string()).as_str())
      .replace("fn()", format!("fn({})", inputs_unnamed).as_str())
      .parse()
      .unwrap();
  } else {
    panic!("Fatal error in hook macro")
  }
}

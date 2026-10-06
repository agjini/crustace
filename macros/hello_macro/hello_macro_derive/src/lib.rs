use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::os::unix::prelude::CommandExt;
use std::process::Command;
use syn::{DeriveInput, ItemFn, TraitItemFn, parse_macro_input};
//
// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     // Construct a representation of Rust code as a syntax tree
//     // that we can manipulate.
//     let ast = syn::parse(input).unwrap();
//
//     // Build the trait implementation.
//     impl_hello_macro(&ast)
// }

#[proc_macro_attribute]
pub fn measure(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TraitItemFn = syn::parse(item).unwrap();

    let command = Command::new("whoami").output().unwrap().stdout;
    let who = String::from_utf8(command).unwrap();

    Command::new("notify-send")
        .arg(format!("Salut {}", &who))
        .output()
        .expect("failed to execute process");

    let vv = item.sig.ident.to_string();
    //item.into_token_stream().into()
    quote! {
        fn hello() {
            println!("Hello, {}!", #vv);
        }
    }
    .into()
}

fn impl_hello_macro(p0: &DeriveInput) -> TokenStream {
    let name = p0.ident.clone();
    quote!(
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    )
    .into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let dialect = GenericDialect {};
    let statements = Parser::parse_sql(&dialect, &input.to_string()).unwrap();
    assert_eq!(statements.len(), 1);
    let statement = "pipi";
    quote! {
        #statement
    }
    .into()
}

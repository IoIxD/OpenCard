#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::fmt::Write;

// data layout according to https://hypercard.org/hypercard_file_format_pierre/
use proc_macro::TokenStream;
use proc_macro2::Literal;
use syn::{parse_macro_input, DeriveInput, LitInt, Meta, MetaNameValue};

#[proc_macro_attribute]
pub fn layout(args: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let enum_name = args.to_string();

    let mut result: String = String::new();

    if let syn::Data::Enum(s) = data {
        // start with the enum itself
        result.write_str(
            format!(
                "
        #[derive(Debug)]
        pub struct {} {{}}
        impl {} {{",
                enum_name, enum_name
            )
            .as_str(),
        );
        let mut offset = 0;
        for var in &s.variants {
            let name = var.ident.to_string();
            result.write_str(
                format!("pub fn {}Start() -> usize {{ {:#04x} }}", name, offset).as_str(),
            );
            if let Some(i) = var.attrs.get(0) {
                if let Meta::List(value) = &i.meta {
                    let litint = value.parse_args::<LitInt>();
                    match litint {
                        Ok(num) => match num.base10_parse::<u16>() {
                            Ok(a) => {
                                offset += a;
                            }
                            Err(err) => {
                                panic!("{}", err);
                            }
                        },
                        Err(err) => {
                            panic!("{}", err);
                        }
                    }
                    result.write_str(
                        format!("pub fn {}End() -> usize {{ {:#04x} }}", name, offset).as_str(),
                    );
                }
            } else {
                panic!("Needs ahead attribute")
            }
        }
        // then write the parse functions for it.
        result.write_str("}");
    }

    //println!("{}",result);

    result.as_str().parse().unwrap()
}

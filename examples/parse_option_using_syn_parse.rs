//! Example on using [`syn::parse::Parse`] as a
//! [`ParseOption`](::macroific::attr_parse::ParseOption) implementation

fn main() {
    panic!("Run me with `cargo test --features attr_parse --example parse_option_using_syn_parse`");
}

#[cfg(test)]
mod test {

    use macroific::prelude::*;
    use proc_macro2::Ident;
    use syn::parse::{Parse, ParseStream};
    use syn::parse_quote;

    #[derive(AttributeOptions)]
    struct MyOptions {
        #[attr_opts(default = false)]
        foo: MyOption,
    }

    #[derive(ParseOption)]
    #[attr_opts(from_parse)] // from_parse is the important bit
    struct MyOption {
        value: Ident,
    }

    impl Parse for MyOption {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                value: input.parse()?,
            })
        }
    }

    #[test]
    fn run() {
        let opts = MyOptions::from_attr(parse_quote!(#[anything(foo = bar)])).unwrap();
        assert_eq!(opts.foo.value, "bar");
    }
}

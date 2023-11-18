fn main() {
    panic!("Run me with `cargo test --features attr_parse --example field_with_opts`");
}

#[cfg(test)]
mod test {
    use macroific::attr_parse::FieldsWithOpts;
    use macroific::prelude::*;

    #[derive(AttributeOptions)]
    struct Options {
        #[attr_opts(default = false)]
        bar: syn::LitStr,
    }

    #[test]
    fn main() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct Foo {
                #[my_attr(bar = "qux")]
                baz: &'static str,
            }
        };
        let data = input.data.extract_struct().unwrap();

        // Alternatively, `FieldWithOpts` can be constructed the same way from a syn `Field`
        let fields = FieldsWithOpts::<Options>::from_attr_name(data.fields, "my_attr")
            .expect("Error parsing fields");

        let first_field = match fields {
            FieldsWithOpts::Named {
                fields,
                brace_token: _,
            } => {
                assert_eq!(fields.len(), 1);
                fields.into_iter().next().unwrap()
            }
            _ => panic!("Expected named fields"),
        };

        assert_eq!(&*first_field.options.bar.value(), "qux");
        assert_eq!(first_field.field.ident.unwrap(), "baz");
    }
}

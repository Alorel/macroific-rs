use macroific_core::core_ext::MacroificCoreIdentExt;
use macroific_core::elements::{ImplFor, ModulePrefix};
use syn::Token;

use super::{BASE, PRIVATE};

common_impl!(ParseOptionDerive "ParseOption");

impl ToTokens for ParseOptionDerive {
    common_impl!(to_tokens);

    fn to_token_stream(&self) -> TokenStream {
        let fields = match self.named_fields() {
            Ok(fields) => fields,
            Err(delim) => return self.render_empty(delim),
        };

        let mut tokens = self.impl_for();

        let fn_body = Group::new(Delimiter::Brace, {
            let indexed_fields = super::indexed_fields(fields);

            let matches = indexed_fields.clone()
                .map(move |(option_var_name, field)| {
                    let mut stream = field.resolved_label().into_token_stream();
                    <Token![=>]>::default().to_tokens(&mut stream);

                    stream.append(Group::new(
                        Delimiter::Brace,
                        quote! { #PRIVATE::decode_parse_option_field(&mut #option_var_name, ident, value_source) },
                    ));

                    stream
                });

            let mut out = super::nones(fields);

            out.append_all(quote! {
                // Provided ident, but no value, then continued to provide the next ident
                if !parse.peek(::syn::Token![,]) {
                    for result in #PRIVATE::iterate_option_meta(parse)? {
                        let (ident, value_source) = result?;

                        match ::std::string::ToString::to_string(&ident).as_str() {
                            #(#matches)*
                            other => return #RESULT::Err(::syn::Error::new(::syn::spanned::Spanned::span(&ident), format!("Unrecognised attribute: `{other}`"))),
                        }?;

                    }
                }
            });

            // let mut out = quote! {
            //     let meta_list: ::syn::MetaList = ::syn::parse::Parse::parse(parse)?;
            // };
            // out.append_all(super::nones(fields));
            // out.append_all(super::matches_parse_nested_meta(
            //     indexed_fields.clone(),
            //     Ident::create("meta_list")
            // ));
            RESULT.with_ident(Ident::create("Ok")).to_tokens(&mut out);

            out.append(Group::new(Delimiter::Parenthesis, {
                let mut out = <Token![Self]>::default().into_token_stream();
                let unwraps = super::unwraps(
                    indexed_fields,
                    &quote! {
                        ::proc_macro2::Span::call_site()
                    },
                );
                out.append(unwraps);
                out
            }));

            out
        });

        // Struct body
        tokens.append(Group::new(Delimiter::Brace, {
            let mut signature = quote! {
                fn from_stream(parse: ::syn::parse::ParseStream) -> ::syn::Result<Self>
            };
            signature.append(fn_body);
            signature
        }));

        let impl_parse = ImplFor::new(
            self.generics(),
            ModulePrefix::new(&["syn", "parse", "Parse"]),
            self.ident(),
        );

        tokens.append_all(quote! {
            #impl_parse {
                #INLINE
                fn parse(parse: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                    #BASE::ParseOption::from_stream(parse)
                }
            }
        });

        tokens
    }
}

impl ParseOptionDerive {
    #[allow(clippy::needless_pass_by_value)]
    fn render_empty_body(ending: Option<Group>) -> TokenStream {
        quote! {
            #INLINE
            fn from_stream(_: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                #RESULT::Ok(Self #ending)
            }
        }
    }
}

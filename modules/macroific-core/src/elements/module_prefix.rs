//! A `const`-table module prefix, e.g. `::your_crate::__private`.

use std::ops::{Deref, Index};
use std::{fmt, iter, slice};

use crate::core_ext::*;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::Token;

/// Prefix for [`::core::option::Option`].
pub const OPTION: ModulePrefix<'static> = ModulePrefix::new(&["core", "option", "Option"]);

/// Prefix for [`::core::result::Result`].
pub const RESULT: ModulePrefix<'static> = ModulePrefix::new(&["core", "result", "Result"]);

/// A `const`-table module prefix, e.g. `::your_crate::__private`.
///
/// ```
/// # use macroific_core::elements::*;
/// # use syn::parse_quote;
/// # use quote::{quote, ToTokens};
/// #
/// let prefixed = ModulePrefix::new(&["foo", "bar"]);
/// let prefixed_stream = prefixed.to_token_stream().to_string();
/// assert_eq!(prefixed_stream, ":: foo :: bar");
///
/// let unprefixed = prefixed.with_leading_sep(false);
/// let unprefixed_stream = unprefixed.to_token_stream().to_string();
/// assert_eq!(unprefixed_stream, "foo :: bar");
///
/// // Display is also implemented
/// assert_eq!(prefixed.to_string(), prefixed_stream);
/// assert_eq!(unprefixed.to_string(), unprefixed_stream);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
#[cfg(feature = "module-prefix")]
pub struct ModulePrefix<'a> {
    path: &'a [&'a str],
    leading_sep: bool,
}

/// A chained [`ModulePrefix`] separated by `::`.
#[cfg(feature = "module-prefix")]
pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<'a> ModulePrefix<'a> {
    /// Create a new `ModulePrefix` from a slice of segments.
    #[inline]
    #[must_use]
    pub const fn new(segments: &'a [&'a str]) -> Self {
        Self {
            path: segments,
            leading_sep: true,
        }
    }

    /// `true` (default) will include the leading `::`, `false` will omit it.
    #[inline]
    #[must_use]
    pub const fn with_leading_sep(mut self, leading_sep: bool) -> Self {
        self.leading_sep = leading_sep;
        self
    }
}

impl<'a> IntoIterator for ModulePrefix<'a> {
    type Item = &'a str;
    type IntoIter = iter::Copied<slice::Iter<'a, &'a str>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.path.iter().copied()
    }
}

impl ToTokens for ModulePrefix<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut iter = self.into_iter();

        if !self.leading_sep {
            if let Some(first) = iter.next() {
                tokens.append(Ident::create(first));
            } else {
                return;
            }
        }

        let sep = <Token![::]>::default();

        for segment in iter {
            sep.to_tokens(tokens);
            tokens.append(Ident::create(segment));
        }
    }
}

impl fmt::Display for ModulePrefix<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.into_iter();
        let Some(first) = iter.next() else {
            return Ok(());
        };

        if self.leading_sep {
            f.write_str(":: ")?;
            f.write_str(first)?;
        } else {
            f.write_str(first)?;
        }

        for item in iter {
            f.write_str(" :: ")?;
            f.write_str(item)?;
        }

        Ok(())
    }
}

/// # Example
///
/// ```
/// # use macroific_core::elements::ModulePrefix;
/// #
/// fn accept_str_slice(_: &[&str]) {}
///
/// let prefix = ModulePrefix::new(&["foo", "bar"]);
/// accept_str_slice(&prefix); // derefs fine
/// ```
impl<'a> Deref for ModulePrefix<'a> {
    type Target = [&'a str];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

/// # Example
///
/// ```
/// # use macroific_core::elements::ModulePrefix;
/// #
/// fn accept_str_slice(_: &[&str]) {}
///
/// let prefix = ModulePrefix::new(&["foo", "bar"]);
/// accept_str_slice(prefix.as_ref());
/// ```
impl<'a> AsRef<[&'a str]> for ModulePrefix<'a> {
    #[inline]
    fn as_ref(&self) -> &[&'a str] {
        self.path
    }
}

/// # Example
///
/// ```
/// # use macroific_core::elements::ModulePrefix;
/// #
/// let prefix = ModulePrefix::new(&["foo", "bar"]);
/// assert_eq!(&prefix[0], "foo");
/// assert_eq!(&prefix[1], "bar");
/// ```
impl Index<usize> for ModulePrefix<'_> {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.path[index]
    }
}

impl ModulePrefix<'_> {
    /// Chain the path with another segment - typically an [`Ident`], [`Path`](syn::Path), or
    /// another [`ModulePrefix`] or [`Chain`].
    ///
    /// # Example
    ///
    /// ```
    /// # use macroific_core as macroific;
    /// # use macroific::core_ext::*;
    /// # use macroific::elements::ModulePrefix;
    /// # use syn::{DeriveInput, parse_quote};
    /// # use proc_macro2::{Ident, TokenStream};
    /// # use quote::quote;
    /// #
    /// const MAIN_MODULE: ModulePrefix<'static> = ModulePrefix::new(&["my_crate", "some_module"]);
    ///
    /// // Simplified view of a macro
    /// fn parse() -> DeriveInput {
    ///   parse_quote!(struct Foo(u8);) // would be an actual token stream in a derive macro
    /// }
    ///
    /// # //noinspection RsConstantConditionIf
    /// fn to_tokens(input: DeriveInput) -> TokenStream {
    ///   const SOME_CONDITION: bool = true;
    ///   const ANOTHER_CONDITION: bool = false;
    ///
    ///   let submodule = MAIN_MODULE.chain(Ident::create(if SOME_CONDITION {
    ///     "foo"
    ///   } else {
    ///     "bar"
    ///   }));
    ///
    ///   let trait_name = submodule.chain(Ident::create(if ANOTHER_CONDITION {
    ///     "Qux"
    ///   } else {
    ///     "Baz"
    ///   }));
    ///
    ///   // Implement some imaginary trait forwarding
    ///   let ident = &input.ident;
    ///   quote! {
    ///     impl #trait_name for #ident {
    ///       fn exec(self) {
    ///         #trait_name::exec(self.inner);
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// let input = parse();
    /// let output = to_tokens(input);
    ///
    /// assert_eq!(output.to_string(), "impl :: my_crate :: some_module :: foo :: Baz for Foo { fn exec (self) { :: my_crate :: some_module :: foo :: Baz :: exec (self . inner) ; } }");
    /// ```
    ///
    /// Output in a bit more readable format:
    ///
    #[cfg_attr(doctest, doc = " ````no_test")]
    /// ```
    /// impl ::my_crate::some_module::foo::Baz for Foo {
    ///     fn exec(self) {
    ///         ::my_crate::some_module::foo::Baz::exec(self.inner);
    ///     }
    /// }
    /// ````
    pub const fn chain<T>(self, next_segment: T) -> Chain<Self, T>
    where
        T: ToTokens,
    {
        Chain {
            a: self,
            b: next_segment,
        }
    }
}

impl<A, B> Chain<A, B> {
    /// Chain the path further. See [`ModulePrefix::chain`].
    pub const fn chain<C>(self, next_segment: C) -> Chain<Self, C>
    where
        C: ToTokens,
    {
        Chain {
            a: self,
            b: next_segment,
        }
    }
}

impl<A: ToTokens, B: ToTokens> ToTokens for Chain<A, B> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.a.to_tokens(tokens);
        <Token![::]>::default().to_tokens(tokens);
        self.b.to_tokens(tokens);
    }
}

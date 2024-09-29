use std::fmt;
use std::iter::Copied;
use std::ops::{Deref, Index};

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::token::PathSep;

use crate::core_ext::*;

/// A module prefix, e.g. `::your_crate::__private`.
///
/// ```
/// # use macroific_core::elements::*;
/// # use proc_macro2::*;
/// # use syn::*;
/// # use quote::quote;
///
/// // `Display` implementation comes from `ToTokens`
/// const PREFIX: ModulePrefix = ModulePrefix::new(&["foo", "bar"]);
///
/// assert_eq!(PREFIX.to_string(), ":: foo :: bar");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct ModulePrefix<'a>(&'a [&'a str]);
impl ModulePrefix<'static> {
    /// Prefix for [`Option`]
    pub const OPTION: Self = Self::new(&["core", "option", "Option"]);

    /// Prefix for [`Result`]
    pub const RESULT: Self = Self::new(&["core", "result", "Result"]);
}

impl<'a> ModulePrefix<'a> {
    /// Create a new `ModulePrefix` from a slice of segments.
    #[inline]
    #[must_use]
    pub const fn new(segments: &'a [&'a str]) -> Self {
        Self(segments)
    }
}

impl<'a> fmt::Display for ModulePrefix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_token_stream(), f)
    }
}

impl<'a> IntoIterator for &ModulePrefix<'a> {
    type Item = &'a str;
    type IntoIter = Copied<core::slice::Iter<'a, &'a str>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl<'a> ToTokens for ModulePrefix<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sep = PathSep::default();

        for segment in self {
            sep.to_tokens(tokens);
            tokens.append(Ident::create(segment));
        }
    }
}

impl<'a> Deref for ModulePrefix<'a> {
    type Target = [&'a str];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Index<usize> for ModulePrefix<'a> {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.0[index]
    }
}

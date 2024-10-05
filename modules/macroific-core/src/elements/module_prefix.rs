//! A `const`-table module prefix, e.g. `::your_crate::__private`.

use std::ops::{Deref, Index};
use std::{array, fmt};

use crate::core_ext::*;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::Token;

/// Prefix for [`::core::option::Option`].
pub const OPTION: ModulePrefix<'static, 3> = ModulePrefix::new(["core", "option", "Option"]);

/// Prefix for [`::core::result::Result`].
pub const RESULT: ModulePrefix<'static, 3> = ModulePrefix::new(["core", "result", "Result"]);

/// A `const`-table module prefix, e.g. `::your_crate::__private`.
///
/// ```
/// # use macroific_core::elements::*;
/// # use syn::parse_quote;
/// # use quote::{quote, ToTokens};
/// #
/// const PREFIXED: ModulePrefix<'static, 2> = ModulePrefix::new(["foo", "bar"]);
/// const UNPREFIXED: ModulePrefix<'static, 2> = ModulePrefix::new(["foo", "bar"])
///   .with_leading_sep(false);
///
/// let tokens_prefixed = PREFIXED.to_token_stream().to_string();
/// let tokens_unprefixed = UNPREFIXED.to_token_stream().to_string();
///
/// assert_eq!(tokens_prefixed, ":: foo :: bar");
/// assert_eq!(tokens_unprefixed, "foo :: bar");
///
/// // Display is also implemented
/// assert_eq!(PREFIXED.to_string(), tokens_prefixed);
/// assert_eq!(UNPREFIXED.to_string(), tokens_unprefixed);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
#[cfg(feature = "module-prefix")]
pub struct ModulePrefix<'a, const LEN: usize> {
    path: [&'a str; LEN],
    leading_sep: bool,
}

impl<'a, const LEN: usize> ModulePrefix<'a, LEN> {
    /// Create a new `ModulePrefix` from a slice of segments.
    #[inline]
    #[must_use]
    pub const fn new(segments: [&'a str; LEN]) -> Self {
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

impl<'a, const LEN: usize> IntoIterator for ModulePrefix<'a, LEN> {
    type Item = &'a str;
    type IntoIter = array::IntoIter<&'a str, LEN>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<'a, const LEN: usize> ToTokens for ModulePrefix<'a, LEN> {
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

impl<'a, const LEN: usize> fmt::Display for ModulePrefix<'a, LEN> {
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
/// let prefix = ModulePrefix::new(["foo", "bar"]);
/// accept_str_slice(&prefix); // derefs fine
/// ```
impl<'a, const LEN: usize> Deref for ModulePrefix<'a, LEN> {
    type Target = [&'a str];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

/// # Example
///
/// ```
/// # use macroific_core::elements::ModulePrefix;
/// #
/// let prefix = ModulePrefix::new(["foo", "bar"]);
/// assert_eq!(&prefix[0], "foo");
/// assert_eq!(&prefix[1], "bar");
/// ```
impl<'a, const LEN: usize> Index<usize> for ModulePrefix<'a, LEN> {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.path[index]
    }
}

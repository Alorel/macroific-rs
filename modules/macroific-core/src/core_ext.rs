//! Extension traits

use proc_macro2::{Ident, Punct, Spacing, Span};
use sealed::sealed;
use std::fmt::Display;
use syn::Error;

/// [`Ident`] extensions
#[sealed]
pub trait MacroificCoreIdentExt {
    /// Shorthand for `Ident::new(name, Span::call_site())`
    fn create(name: &str) -> Self;
}

/// [`Punct`] extensions
#[sealed]
pub trait MacroificCorePunctExt {
    /// Create a new [`Punct`] with [`Spacing::Alone`]
    fn new_alone(ch: char) -> Self;

    /// Create a new [`Punct`] with [`Spacing::Joint`]
    fn new_joint(ch: char) -> Self;
}

/// [`Error`] extensions
#[sealed]
pub trait MacroificCoreErrorExt {
    /// Shorthand for [`Error::new(Span::call_site(), msg)`](Error::new).
    fn call_site<T: Display>(msg: T) -> Self;
}

#[sealed]
impl MacroificCorePunctExt for Punct {
    #[inline]
    fn new_alone(ch: char) -> Self {
        Self::new(ch, Spacing::Alone)
    }

    #[inline]
    fn new_joint(ch: char) -> Self {
        Self::new(ch, Spacing::Joint)
    }
}

#[sealed]
impl MacroificCoreIdentExt for Ident {
    #[inline]
    fn create(name: &str) -> Self {
        Self::new(name, Span::call_site())
    }
}

#[sealed]
impl MacroificCoreErrorExt for Error {
    fn call_site<T: Display>(msg: T) -> Self {
        Self::new(Span::call_site(), msg)
    }
}

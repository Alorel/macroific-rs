//! Various tokenisable elements

#[cfg(feature = "generic-impl")]
pub use generic_impl::GenericImpl;
pub use module_prefix::ModulePrefix;

#[cfg(feature = "generic-impl")]
pub mod generic_impl;
pub mod module_prefix;

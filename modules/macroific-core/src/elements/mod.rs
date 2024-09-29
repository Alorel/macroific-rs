//! Various tokenisable elements

#[cfg(feature = "generic-impl")]
pub use generic_impl::GenericImpl;

#[cfg(feature = "module-prefix")]
pub use module_prefix::ModulePrefix;

#[cfg(feature = "generic-impl")]
pub mod generic_impl;

#[cfg(feature = "module-prefix")]
pub mod module_prefix;

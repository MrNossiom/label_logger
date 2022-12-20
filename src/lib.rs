#![warn(
	clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::cargo
)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/MrNossiom/label-logger/main/logo.png")]
#![doc = include_str!("../README.md")]

#[cfg(feature = "dialoguer")]
pub mod dialoguer;
#[cfg(feature = "indicatif")]
pub mod indicatif;
mod log;
mod macros;
mod util;

// Re-exports
#[cfg(feature = "dialoguer")]
pub use crate::dialoguer::LabelTheme;
#[cfg(feature = "indicatif")]
pub use crate::indicatif::label_theme;
pub use crate::log::{pretty_output, println_label, OutputLabel};
pub use crate::macros::*;
pub use console;

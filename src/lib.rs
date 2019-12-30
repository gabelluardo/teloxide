#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]
#![allow(clippy::match_bool)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub use bot::Bot;
pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

mod bot;
pub mod dispatching;
pub mod requests;
pub mod types;

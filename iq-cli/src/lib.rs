//! Command line application microframework which supports command-line
//! option parsing, basic terminal management, and Cargo-like status output.
//!
//! # Option Parser
//!
//! Please see the documentation for the `options` module.
//!
//! # Status Macros
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//!
//! // Print an error message
//! status_err!("something bad happened");
//!
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//!
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

#![crate_name = "iq_cli"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/iq-cli/0.1.0")]

extern crate failure;
#[allow(unknown_lints, unused_imports, useless_attribute)]
#[macro_use]
extern crate iq_cli_derive;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "log")]
pub extern crate log;
#[cfg(feature = "simplelog")]
extern crate simplelog;
extern crate term;

#[cfg(all(test, feature = "options"))]
#[macro_use]
extern crate assert_matches;

#[cfg(feature = "simplelog")]
use simplelog::{CombinedLogger, LevelFilter, TermLogger};
pub use term::color::{self, Color};

mod error;
#[cfg(any(feature = "errors", feature = "status"))]
pub mod macros;
#[cfg(feature = "options")]
pub mod options;
mod shell;

pub use error::Error;
#[cfg(feature = "options")]
pub use options::Options;
pub use shell::{config, status, ColorConfig, Stream};

/// Initialize a command-line app with the given options
// TODO: better API for this
#[allow(unused_variables)]
pub fn init(color_config: ColorConfig, verbose: bool) {
    config(color_config);
    #[cfg(feature = "simplelog")]
    init_logging(verbose);
}

/// Initialize the logging subsystem (i.e. simplelog)
#[cfg(feature = "simplelog")]
fn init_logging(verbose: bool) {
    let level_filter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let config = simplelog::Config::default();

    if let Some(logger) = TermLogger::new(level_filter, config) {
        CombinedLogger::init(vec![logger]).unwrap()
    }
}
//! # HAL for the ATSAM3X family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the ATSAM3X family of
//! microcontrollers.
//!
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
//!
//! # Usage
//!
//! ## Building an application (binary crate)
//!
//! A detailed usage guide can be found in the [README]
//!
//! supported microcontrollers are:
//!
//! - atsam3x4c
//! - atsam3x4e
//! - atsam3x8c
//! - atsam3x8e
//!
//! ## Usage
//!
//! This crate supports multiple microcontrollers in the
//! atsam3x family. Which specific microcontroller you want to build for has to be
//! specified with a feature, for example `atsam3x4c`.
//!
//! If no microcontroller is specified, the crate will not compile.
//!
//! The currently supported variants are
//!
//! - `atsam3x4c`
//! - `atsam3x4e`
//! - `atsam3x8c`
//! - `atsam3x8e`
//!
//! ## Commonly used setup
//! Almost all peripherals require references to some registers in `RCC` and `AFIO`. The following
//! code shows how to set up those registers
//!
//! ```rust
//! // Get access to the device specific peripherals from the peripheral access crate
//! let dp = pac::Peripherals::take().unwrap();
//!
//! // Take ownership over the raw flash and pmc devices and convert them into the corresponding
//! // HAL structs
//! let mut flash = dp.EFC0.constrain();
//! let mut pmc = dp.PMC.constrain();
//!
//! // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
//! // `clocks`
//! let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!
//! // Prepare the alternate function I/O registers
//! let mut afio = dp.AFIO.constrain();
//! ```
//!
//! ## Usage examples
//!
//! See the [examples] folder.
//!
//! Most of the examples require the following additional dependencies
//! ```toml
//! [dependencies]
//! embedded-hal = "0.2.3"
//! nb = "0.1.2"
//! cortex-m = "0.6.2"
//! cortex-m-rt = "0.6.11"
//! # Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
//! panic-halt = "0.2.0"
//! ```
//!
//! [examples]: https://github.com/emcrates-rs/atsam3xxx-hal-rs/tree/v0.7.0/examples
//! [README]: https://github.com/emcrates-rs/atsam3xxx-hal-rs/tree/v0.7.0

#![no_std]

// If no target specified, print error message.
#[cfg(not(any(
    feature = "atsam3x4c",
    feature = "atsam3x4e",
    feature = "atsam3x8c",
    feature = "atsam3x8e",
)))]
compile_error!("Target not found. A `--features <target-name>` is required.");

// If any two or more targets are specified, print error message.
#[cfg(any(
    all(feature = "atsam3x4c", feature = "atsam3x4e"),
    all(feature = "atsam3x4c", feature = "atsam3x8c"),
    all(feature = "atsam3x4c", feature = "atsam3x8e"),
    all(feature = "atsam3x4e", feature = "atsam3x8c"),
    all(feature = "atsam3x4e", feature = "atsam3x8e"),
    all(feature = "atsam3x8c", feature = "atsam3x8e"),
))]
compile_error!(
    "Multiple targets specified. Only a single `--features <target-name>` can be specified."
);

//Enable HAL-Traits
#[cfg(feature = "device-selected")]
use embedded_hal as hal;

//Enable PAC for specified feature
#[cfg(feature = "atsam3x4c")]
pub use atsam3x::atsam3x4c as pac;

#[cfg(feature = "atsam3x4e")]
pub use atsam3x::atsam3x4e as pac;

#[cfg(feature = "atsam3x8c")]
pub use atsam3x::atsam3x8c as pac;

#[cfg(feature = "atsam3x8e")]
pub use atsam3x::atsam3x8e as pac;

// HAL Modules
#[cfg(feature = "device-selected")]
pub mod eefc;



#[cfg(feature = "device-selected")]
mod sealed {
    pub trait Sealed {}
}
#[cfg(feature = "device-selected")]
use sealed::Sealed;
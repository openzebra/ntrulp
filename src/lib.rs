#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod encode;
pub mod key;
pub mod math;
pub mod ntru;
pub mod params;
pub mod poly;
pub mod rng;

#[cfg(feature = "std")]
pub mod compress;

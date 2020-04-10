//! Driver for the HTPA32x32 an Thermopile Array from Heimann

#![cfg_attr(not(test), no_std)] // instead of #![no_std] to be able to test with embedded-hal-mock
// #![deny(missing_debug_implementations)]
//#![deny(missing_docs)] // just allow for missing docs right now
#![allow(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]


// use embedded_hal as hal;

//use libm;
// use bit_field::BitField;

// use crate::hal::blocking::delay::DelayMs;
// use crate::hal::blocking::i2c::{Write, WriteRead};
// use embedded_hal::blocking::i2c::Read;


#[cfg(test)]
mod tests;


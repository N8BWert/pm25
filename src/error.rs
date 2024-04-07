//!
//! Error from operating the PM25
//! 

use core::fmt::Debug;

pub enum PM25Error<E: Debug> {
    InvalidAddress{expected: u8, found: u8},
    InvalidChecksum{expected: u16, found: u16},
    I2C(E),
}
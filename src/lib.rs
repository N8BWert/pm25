//!
//! Rust driver for the Adafruit PM2.5 Air Quality Sensor
//! 

#![no_std]

use core::fmt::Debug;

use embedded_hal::i2c::{I2c, SevenBitAddress};

pub mod error;
use error::PM25Error;

pub mod reading;
pub use reading::PM25Reading;
use reading::PM25ReadingBuilder;

// The I2C Address of the Module
pub const PM25_ADDRESS: u8 = 0x12;

pub struct PM25Driver<I2C, E> where
    I2C: I2c<SevenBitAddress, Error=E>,
    E: Debug {
    i2c: I2C,
}

impl<I2C, E> PM25Driver<I2C, E> where
    I2C: I2c<SevenBitAddress, Error=E>,
    E: Debug {
    /// Initialize a new Driver
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
        }
    }

    pub fn read_data(&mut self) -> Result<PM25Reading, PM25Error<E>> {
        let mut buffer = [0u8; 32];
        self.i2c.read(PM25_ADDRESS, &mut buffer).map_err(PM25Error::I2C)?;

        if buffer[0] != PM25_ADDRESS {
            return Err(PM25Error::InvalidAddress { expected: PM25_ADDRESS, found: buffer[0] });
        }

        // Validate Checksum
        let mut sum = 0;
        for i in 0..30 {
            sum += buffer[i] as u16;
        }

        let expected_checksum = u16::from_le_bytes(buffer[30..32].try_into().unwrap());
        if sum != expected_checksum {
            return Err(PM25Error::InvalidChecksum { expected: expected_checksum, found: sum })
        }

        let reading = PM25ReadingBuilder::new()
            .pm10_standard(buffer[4..6].try_into().unwrap())
            .pm25_standard(buffer[6..8].try_into().unwrap())
            .pm100_standard(buffer[8..10].try_into().unwrap())
            .pm10_env(buffer[10..12].try_into().unwrap())
            .pm25_env(buffer[12..14].try_into().unwrap())
            .pm100_env(buffer[14..16].try_into().unwrap())
            .particles_03um(buffer[16..18].try_into().unwrap())
            .particles_05um(buffer[18..20].try_into().unwrap())
            .particles_10um(buffer[20..22].try_into().unwrap())
            .particles_25um(buffer[22..24].try_into().unwrap())
            .particles_50um(buffer[24..26].try_into().unwrap())
            .particles_100um(buffer[26..28].try_into().unwrap())
            .build();

        Ok(reading)
    }
}

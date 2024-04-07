//!
//! Air Quality Reading from the PM25.
//!

/// Air Quality Reading from the PM25
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PM25Reading {
    // standard PM1.0 reading
    pm10_standard: u16,
    // standard PM2.5 reading
    pm25_standard: u16,
    // standard PM10.0 reading
    pm100_standard: u16,
    // environmental PM1.0 reading
    pm10_env: u16,
    // environmental PM2.5 reading
    pm25_env: u16,
    // environmental PM10.0 reading
    pm100_env: u16,
    // 0.3um Particle Count
    particles_03um: u16,
    // 0.5um Particle Count
    particles_05um: u16,
    // 1.0um Particle Count
    particles_10um: u16,
    // 2.5um Particle Count
    particles_25um: u16,
    // 5.0um Particle Count
    particles_50um: u16,
    // 10.0um Particle Count
    particles_100um: u16,
}

pub struct PM25ReadingBuilder {
    pm10_standard: Option<u16>,
    pm25_standard: Option<u16>,
    pm100_standard: Option<u16>,
    pm10_env: Option<u16>,
    pm25_env: Option<u16>,
    pm100_env: Option<u16>,
    particles_03um: Option<u16>,
    particles_05um: Option<u16>,
    particles_10um: Option<u16>,
    particles_25um: Option<u16>,
    particles_50um: Option<u16>,
    particles_100um: Option<u16>,
}

impl PM25ReadingBuilder {
    pub fn new() -> Self {
        Self {
            pm10_standard: None,
            pm25_standard: None,
            pm100_standard: None,
            pm10_env: None,
            pm25_env: None,
            pm100_env: None,
            particles_03um: None,
            particles_05um: None,
            particles_10um: None,
            particles_25um: None,
            particles_50um: None,
            particles_100um: None,
        }
    }

    pub fn pm10_standard(mut self, value: [u8; 2]) -> Self {
        self.pm10_standard = Some(u16::from_le_bytes(value));
        self
    }

    pub fn pm25_standard(mut self, value: [u8; 2]) -> Self {
        self.pm25_standard = Some(u16::from_le_bytes(value));
        self
    }

    pub fn pm100_standard(mut self, value: [u8; 2]) -> Self {
        self.pm100_standard = Some(u16::from_le_bytes(value));
        self
    }

    pub fn pm10_env(mut self, value: [u8; 2]) -> Self {
        self.pm10_env = Some(u16::from_le_bytes(value));
        self
    }

    pub fn pm25_env(mut self, value: [u8; 2]) -> Self {
        self.pm25_env = Some(u16::from_le_bytes(value));
        self
    }

    pub fn pm100_env(mut self, value: [u8; 2]) -> Self {
        self.pm100_env = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_03um(mut self, value: [u8; 2]) -> Self {
        self.particles_03um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_05um(mut self, value: [u8; 2]) -> Self {
        self.particles_05um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_10um(mut self, value: [u8; 2]) -> Self {
        self.particles_10um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_25um(mut self, value: [u8; 2]) -> Self {
        self.particles_25um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_50um(mut self, value: [u8; 2]) -> Self {
        self.particles_50um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn particles_100um(mut self, value: [u8; 2]) -> Self {
        self.particles_100um = Some(u16::from_le_bytes(value));
        self
    }

    pub fn build(self) -> PM25Reading {
        let pm10_standard = match self.pm10_standard {
            Some(value) => value,
            None => u16::MAX,
        };

        let pm25_standard = match self.pm25_standard {
            Some(value) => value,
            None => u16::MAX,
        };

        let pm100_standard = match self.pm100_standard {
            Some(value) => value,
            None => u16::MAX,
        };

        let pm10_env = match self.pm10_env {
            Some(value) => value,
            None => u16::MAX,
        };

        let pm25_env = match self.pm25_env {
            Some(value) => value,
            None => u16::MAX,
        };

        let pm100_env = match self.pm100_env {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_03um = match self.particles_03um {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_05um = match self.particles_05um {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_10um = match self.particles_10um {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_25um = match self.particles_25um {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_50um = match self.particles_50um {
            Some(value) => value,
            None => u16::MAX,
        };

        let particles_100um = match self.particles_100um {
            Some(value) => value,
            None => u16::MAX,
        };

        PM25Reading {
            pm10_standard,
            pm25_standard,
            pm100_standard,
            pm10_env,
            pm25_env,
            pm100_env,
            particles_03um,
            particles_05um,
            particles_10um,
            particles_25um,
            particles_50um,
            particles_100um,
        }
    }
}
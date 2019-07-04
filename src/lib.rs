use std::fmt::{Display, Formatter};
use std::{error, fmt, result};

mod device;

#[macro_use]
mod macros;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidChannelNumber(u8),
    Peripheral(rppal::spi::Error),
    UnsupportedChannel(Channel),
    UnsupportedDifferentialCombination(Channel, Channel),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::InvalidChannelNumber(channel) => write!(f, "Invalid channel number {}", channel),
            Error::Peripheral(e) => e.fmt(f),
            Error::UnsupportedChannel(channel) => write!(f, "Unsupported channel {}", channel.0),
            Error::UnsupportedDifferentialCombination(plus_channel, minus_channel) => write!(
                f,
                "Unsupported channel combination for differential mode: channels {} and {}",
                plus_channel.0, minus_channel.0
            ),
        }
    }
}

impl From<::rppal::spi::Error> for Error {
    fn from(error: rppal::spi::Error) -> Self {
        Error::Peripheral(error)
    }
}

/// The raw reading obtained from the ADC.
pub struct Reading {
    value: u16,
    range: u16,
}

impl Reading {
    pub(crate) fn new(value: u16, range: u16) -> Self {
        Self { value, range }
    }

    /// The raw value read from the ADC, from 0 to $range - 1.
    pub fn value(&self) -> u16 {
        self.value
    }

    /// The range of the value, the maximum possible value.
    pub fn range(&self) -> u16 {
        self.range
    }

    /// The raw value interpreted as a fraction.
    pub fn as_fraction(&self) -> f64 {
        f64::from(self.value) / f64::from(2_i32.pow(12) - 1)
    }
}

/// Chip's channel to be queried.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Channel(pub(crate) u8);

impl Channel {
    pub fn new(n: u8) -> Result<Self> {
        if n > 7 {
            return Err(Error::InvalidChannelNumber(n));
        }

        Ok(Self(n))
    }
}

declare_mcp3xxx!(Mcp3002, "MCP3002 10-bit 2-channel A/D converter.", 10, 2);
declare_mcp3xxx!(Mcp3004, "MCP3004 10-bit 4-channel A/D converter.", 10, 4);
declare_mcp3xxx!(Mcp3008, "MCP3008 10-bit 8-channel A/D converter.", 10, 8);
declare_mcp3xxx!(Mcp3202, "MCP3202 12-bit 2-channel A/D converter.", 12, 2);
declare_mcp3xxx!(Mcp3204, "MCP3204 12-bit 4-channel A/D converter.", 12, 4);
declare_mcp3xxx!(Mcp3208, "MCP3208 12-bit 8-channel A/D converter.", 12, 8);
declare_mcp3xxx!(Mcp3302, "MCP3302 13-bit 4-channel A/D converter.", 13, 4);
declare_mcp3xxx!(Mcp3304, "MCP3304 13-bit 8-channel A/D converter.", 13, 8);

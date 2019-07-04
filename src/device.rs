use crate::{Channel, Error, Reading, Result};
use rppal::spi;
use rppal::spi::Spi;

#[derive(Copy, Clone)]
pub(crate) struct Resolution(pub(crate) u8);

impl Resolution {
    pub(crate) fn new(resolution: u8) -> Result<Self> {
        if resolution == 0 || resolution >= 16 {
            unreachable!();
        }

        Ok(Self(resolution))
    }

    pub fn range(self) -> (u16, u16) {
        (0, 2u16.pow(u32::from(self.0)) - 1)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Channels(pub(crate) u8);

impl Channels {
    pub fn new(num: u8) -> Result<Self> {
        match num {
            2 | 4 | 8 => Ok(Self(num)),
            _ => unreachable!(),
        }
    }

    pub fn bit_size(self) -> u8 {
        match self.0 {
            2 => 1,
            4 | 8 => 3,
            _ => unreachable!(),
        }
    }
}

enum Mode {
    Differential = 0,
    SingleEnded = 1,
}

pub(crate) struct Device {
    spi: Spi,
    channels: Channels,
    resolution: Resolution,
}

impl Device {
    pub(crate) fn new(spi: Spi, channels: Channels, resolution: Resolution) -> Result<Self> {
        spi.set_mode(spi::Mode::Mode0)?;
        spi.set_clock_speed(1_000_000)?;
        spi.set_bit_order(spi::BitOrder::MsbFirst)?;

        Ok(Self {
            spi,
            channels,
            resolution,
        })
    }

    pub(crate) fn single_ended_read(&mut self, channel: Channel) -> Result<Reading> {
        self.check_channel_valid(channel)?;

        self.read(Mode::SingleEnded, channel.0)
    }

    pub(crate) fn differential_read(
        &mut self,
        plus_channel: Channel,
        minus_channel: Channel,
    ) -> Result<Reading> {
        self.check_channel_valid(plus_channel)?;
        self.check_channel_valid(minus_channel)?;

        self.read(
            Mode::Differential,
            match (plus_channel, minus_channel) {
                (Channel(0), Channel(1)) => 0b_000,
                (Channel(1), Channel(0)) => 0b_001,
                (Channel(2), Channel(3)) => 0b_010,
                (Channel(3), Channel(2)) => 0b_011,
                (Channel(4), Channel(5)) => 0b_100,
                (Channel(5), Channel(4)) => 0b_101,
                (Channel(6), Channel(7)) => 0b_110,
                (Channel(7), Channel(6)) => 0b_111,
                (_, _) => {
                    return Err(Error::UnsupportedDifferentialCombination(
                        plus_channel,
                        minus_channel,
                    ))
                }
            },
        )
    }

    fn read(&mut self, mode: Mode, address: u8) -> Result<Reading> {
        // START[1] + MODE[1] + ADDR[1/3] + SAMPLE[1] + NULL[1] + DATA[10-13]
        let size = 1 + 1 + self.channels.bit_size() + 1 + 1 + self.resolution.0;
        let bytes = (f32::from(size) / 8f32).ceil() as u8;

        let command: u32 = 1u32 << u32::from(size - 1)
            | ((mode as u32) << u32::from(size - 2))
            | ((u32::from(address)) << (self.resolution.0 + 2)) as u32;

        let mut tx: Vec<u8> = Vec::with_capacity(bytes as usize);
        for i in (0..bytes).rev() {
            let shift = u32::from(8u8 * i);
            tx.push(((command & (0b_1111_1111u32 << shift)) >> shift) as u8);
        }

        let mut rx: Vec<u8> = Vec::with_capacity(bytes as usize);
        for _ in 0..bytes {
            rx.push(0);
        }

        self.spi.transfer(&mut rx.as_mut_slice(), &tx.as_slice())?;

        let mut result: u32 = 0;
        for (i, byte) in rx.iter().enumerate() {
            result |= (u32::from(*byte) << (u32::from(bytes - 1 - i as u8) * 8)) as u32;
        }

        debug_assert_eq!(result >> u32::from(self.resolution.0), 0);

        Ok(Reading::new(result as u16, self.resolution.range().1))
    }

    fn check_channel_valid(&self, channel: Channel) -> Result<()> {
        if channel.0 >= self.channels.0 {
            return Err(Error::UnsupportedChannel(channel));
        }

        Ok(())
    }
}

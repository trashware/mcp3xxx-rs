macro_rules! declare_mcp3xxx {
    ($name:ident, $doc:tt, $resolution:tt, $channels:tt) => {
        #[doc = $doc]
        pub struct $name {
            device: crate::device::Device,
        }

        impl $name {
            pub fn new(spi: rppal::spi::Spi) -> crate::Result<Self> {
                Ok(Self {
                    device: crate::device::Device::new(
                        spi,
                        crate::device::Channels::new($channels)?,
                        crate::device::Resolution::new($resolution)?,
                    )?,
                })
            }

            /// One-off measurement of a single channel.
            pub fn single_ended_read(
                &mut self,
                channel: crate::Channel,
            ) -> crate::Result<crate::Reading> {
                self.device.single_ended_read(channel)
            }

            /// One-off pseudo-differential measurement of two (plus & minus) channels.
            pub fn differential_read(
                &mut self,
                plus_channel: crate::Channel,
                minus_channel: crate::Channel,
            ) -> crate::Result<crate::Reading> {
                self.device.differential_read(plus_channel, minus_channel)
            }
        }
    };
}

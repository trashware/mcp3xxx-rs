use mcp3xxx::{Channel, Mcp3208};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::str::FromStr;

fn main() {
    let mut mcp3208 =
        Mcp3208::new(Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0).unwrap())
            .unwrap();

    let args = std::env::args().collect::<Vec<String>>();

    let result = mcp3208
        .single_ended_read(Channel::new(u8::from_str(args[1].as_str()).unwrap()).unwrap())
        .unwrap();

    println!(
        "{}/{} = {}",
        result.value(),
        result.range(),
        result.as_fraction()
    );
}

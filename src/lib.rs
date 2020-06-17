#![no_std]
#![allow(non_camel_case_types)]

mod command;

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
use embedded_hal::spi::FullDuplex;
use nb::block;

pub use crate::command::{Register, SpiCommand};

pub struct Drv8305<SPI, NSCS>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin,
{
    spi: SPI,
    nscs: NSCS,
}

impl<SPI, NSCS> Drv8305<SPI, NSCS>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin,
{
    pub fn new(spi: SPI, nscs: NSCS) -> Drv8305<SPI, NSCS> {
        Drv8305 { spi, nscs }
    }

    /// NOTE: On a write command, the returned value is the *previous* register value, not the new one.
    pub fn exec(&mut self, cmd: SpiCommand) -> Result<u16, SPI::Error> {
        let data: u16 = cmd.into();

        self.nscs.set_low();
        // Give the drv at least 50ns to prepare
        cortex_m::asm::delay(8);

        block!(self.spi.send(data))?;
        let res = block!(self.spi.read());

        // Make sure scs is high for at least 500ns between frames
        self.nscs.set_high();
        cortex_m::asm::delay(32);

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

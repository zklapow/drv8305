#![no_std]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate enum_primitive_derive_nostd;

mod command;
pub mod register;

use embedded_hal::digital::v2::StatefulOutputPin;
use embedded_hal::spi::FullDuplex;
use nb::block;

pub use crate::command::SpiCommand;
use crate::register::Register;
use core::convert::Infallible;

pub struct Drv8305<SPI, NSCS>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin<Error = Infallible>, // NOTE: Associated type bounds needed for core::fmt impl to make `unwrap` work... this seems like bad ergonomics?
{
    spi: SPI,
    nscs: NSCS,
}

impl<SPI, NSCS> Drv8305<SPI, NSCS>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin<Error = Infallible>,
{
    pub fn new(spi: SPI, nscs: NSCS) -> Drv8305<SPI, NSCS> {
        Drv8305 { spi, nscs }
    }

    pub fn modify<REG, F>(&mut self, f: F) -> Result<REG, SPI::Error>
    where
        REG: Register,
        F: Fn(REG) -> REG,
    {
        let val: REG = self.read()?;

        let update_reg = f(val);

        let ret_bits = self.exec::<REG>(SpiCommand::write(update_reg.data()))?;
        Ok(REG::parse(ret_bits))
    }

    pub fn read<REG>(&mut self) -> Result<REG, SPI::Error>
    where
        REG: Register,
    {
        let data = self.exec::<REG>(SpiCommand::read())?;

        Ok(REG::parse(data))
    }

    /// NOTE: On a write command, the returned value is the *previous* register value, not the new one.
    fn exec<REG>(&mut self, cmd: SpiCommand<REG>) -> Result<u16, SPI::Error>
    where
        REG: Register,
    {
        let data: u16 = cmd.into();

        self.nscs.set_low().unwrap();
        // Give the drv at least 50ns to prepare
        cortex_m::asm::delay(8);

        block!(self.spi.send(data))?;
        let res = block!(self.spi.read());

        // Make sure scs is high for at least 500ns between frames
        self.nscs.set_high().unwrap();
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

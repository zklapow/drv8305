use crate::register::Register;
use core::marker::PhantomData;

enum RwMode {
    Read = 1,
    Write = 0,
}

const ADDR_MASK: u8 = 0b00001111;
const DATA_MASK: u16 = 0b1111100000000000;

pub struct SpiCommand<REG>
where
    REG: Register,
{
    rw: RwMode,
    data: u16,
    _register: PhantomData<REG>,
}

impl<T> SpiCommand<T>
where
    T: Register,
{
    pub fn write(data: u16) -> SpiCommand<T> {
        assert!(data & DATA_MASK == 0, "Data cannot be more than 11 bytes");

        SpiCommand {
            rw: RwMode::Write,
            _register: PhantomData,
            data,
        }
    }

    pub fn read() -> SpiCommand<T> {
        SpiCommand {
            rw: RwMode::Read,
            _register: PhantomData,
            data: 0,
        }
    }
}

impl<REG> From<SpiCommand<REG>> for u16
where
    REG: Register,
{
    fn from(cmd: SpiCommand<REG>) -> u16 {
        let addr_val = (REG::addr() as u8 & ADDR_MASK) as u16;
        let rw_val: u16 = cmd.rw as u16;
        return (rw_val << 15) | (addr_val as u16) << 11 | cmd.data as u16;
    }
}

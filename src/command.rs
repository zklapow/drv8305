use core::ops::BitXor;

enum RwMode {
    Read = 1,
    Write = 0,
}

const ADDR_MASK: u8 = 0b00001111;
const DATA_MASK: u16 = 0b1111100000000000;

pub enum Register {
    WarningAndWatchdog = 0x1,
    ICFaults = 0x3,
    VdsSenseControl = 0xC,
    VoltageRegulatorControl = 0xB,
    HsGateDriveControl = 0x5,
    LsGateDriveControl = 0x6,
}

pub struct SpiCommand {
    rw: RwMode,
    register: Register,
    data: u16,
}

impl SpiCommand {
    pub fn write(register: Register, data: u16) -> SpiCommand {
        assert!(data & DATA_MASK == 0, "Data cannot be more than 11 bytes");

        SpiCommand {
            rw: RwMode::Write,
            register,
            data,
        }
    }

    pub fn read(register: Register) -> SpiCommand {
        SpiCommand {
            rw: RwMode::Read,
            register,
            data: 0,
        }
    }
}

impl From<SpiCommand> for u16 {
    fn from(cmd: SpiCommand) -> u16 {
        let addr_val = (cmd.register as u8 & ADDR_MASK) as u16;
        let rw_val: u16 = cmd.rw as u16;
        return (rw_val << 15) | (addr_val as u16) << 11 | cmd.data as u16;
    }
}

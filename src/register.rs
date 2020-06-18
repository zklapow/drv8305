use num_traits::cast::FromPrimitive;

pub trait Register {
    fn addr() -> u8;

    fn parse(reg: u16) -> Self;

    fn data(&self) -> u16;
}

#[derive(Copy, Clone)]
pub enum Addrs {
    WarningAndWatchdog = 0x1,
    ICFaults = 0x3,
    VdsSenseControl = 0xC,
    VoltageRegulatorControl = 0xB,
    HsGateDriveControl = 0x5,
    LsGateDriveControl = 0x6,
}

#[derive(Copy, Clone, Primitive)]
pub enum CommOption {
    Diode = 0,
    Active = 1,
}

#[derive(Copy, Clone, Primitive)]
pub enum PwmMode {
    Six = 0,
    Three = 1,
    One = 2,
}

#[derive(Copy, Clone)]
pub struct GateDriveControl {
    bits: u16,
    comm_option: CommOption,
    pwm_mode: PwmMode,
}

// width shifted by start of range from datasheet
const COMM_OPTION_MASK: u16 = 0b1 << 9;
const PWM_MODE_MASK: u16 = 0b11 << 7;

impl GateDriveControl {
    pub fn set_comm_option(mut self, comm_option: CommOption) -> Self {
        self.comm_option = comm_option;
        self
    }

    pub fn set_pwm_mode(mut self, pwm_mode: PwmMode) -> Self {
        self.pwm_mode = pwm_mode;
        self
    }
}

impl Register for GateDriveControl {
    fn addr() -> u8 {
        return 0x7;
    }

    fn parse(reg: u16) -> Self {
        let comm_option_val = CommOption::from_u16((reg & COMM_OPTION_MASK) >> 9).unwrap();
        let pwm_mode_val = PwmMode::from_u16((reg & PWM_MODE_MASK) >> 7).unwrap();

        GateDriveControl {
            bits: reg,
            comm_option: comm_option_val,
            pwm_mode: pwm_mode_val,
        }
    }

    fn data(&self) -> u16 {
        let mut data = self.bits;

        data = (data & !COMM_OPTION_MASK) | ((self.comm_option as u16) << 9);
        data = (data & !PWM_MODE_MASK) | ((self.pwm_mode as u16) << 7);

        data
    }
}

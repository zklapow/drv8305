use num_traits::cast::FromPrimitive;
use num_traits::ToPrimitive;

pub trait Register {
    fn addr() -> u8;

    fn parse(reg: u16) -> Self;

    fn data(&self) -> u16;
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

#[derive(Copy, Clone, Primitive)]
pub enum VdsMode {
    Latched = 0,
    Report = 1,
    Disabled = 2,
}

macro_rules! register {
    (struct $name: ident [$addr: expr] { $($var: ident: $kind: ty [$size: expr, $offset: expr]),+ }) => {
        #[derive(Copy, Clone)]
        pub struct $name {
            bits: u16,
            $($var: $kind,)*
        }

        impl $name {
        $(
            pub fn $var(mut self, $var: $kind) -> Self {
                self.$var = $var;
                self
            }
        )*
        }

        impl Register for $name {
            fn addr() -> u8 {
                return $addr;
            }

            fn parse(reg: u16) -> Self {
                $name {
                    bits: reg,
                    $($var: <$kind>::from_u16((reg & ($size << $offset)) >> $offset).unwrap(),)*
                }
            }

            fn data(&self) -> u16 {
                let mut data = self.bits;

                $(data = (data & !($size << $offset)) | (self.$var.to_u16().unwrap() << $offset);)*

                data
            }
        }
    };
}

register!(struct VdsSenseControl [0xc] {
    vds_level: u16 [0b1111, 3],
    vds_mode: VdsMode [0b111, 0]
});

register!(
    struct GateDriveControl [0x7] {
        comm_option: CommOption [0b1, 9],
        pwm_mode: PwmMode [0b11, 7],
        dead_time: u8 [0b111, 4]
    }
);

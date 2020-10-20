use num_traits::cast::FromPrimitive;
use num_traits::ToPrimitive;

#[cfg(feature = "use-defmt")]
use defmt::Format;

pub trait Register {
    fn addr() -> u8;

    fn parse(reg: u16) -> Self;

    fn data(&self) -> u16;
}

#[derive(Copy, Clone, Primitive, Debug)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum CommOption {
    Diode = 0,
    Active = 1,
}

#[derive(Copy, Clone, Primitive, Debug)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum PwmMode {
    Six = 0,
    Three = 1,
    One = 2,
}

#[derive(Copy, Clone, Primitive, Debug)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum VdsMode {
    Latched = 0,
    Report = 1,
    Disabled = 2,
}

#[derive(Copy, Clone, Primitive, Debug)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum Flag {
    Enabled = 1,
    Disabled = 0,
}

macro_rules! register {
    (struct $name: ident [$addr: expr] { $($var: ident: $kind: ty [$size: expr, $offset: expr]),+ }) => {
        #[derive(Copy, Clone, Debug)]
        #[cfg_attr(feature = "use-defmt", derive(Format))]
        pub struct $name {
            pub bits: u16,
            $(pub $var: $kind,)*
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

register!(struct IcOperation [0x9]{
    en_sns_clamp: Flag [0b1, 7],
    wd_en: Flag [0b1, 3]
});

register!(struct VdsSenseControl [0xc] {
    vds_level: u16 [0b1111, 3],
    vds_mode: VdsMode [0b111, 0]
});

register!(struct ShuntAmplifierControl [0xa] {
    dc_cal_ch3: Flag [0b1, 10],
    dc_cal_ch2: Flag [0b1, 9],
    dc_cal_ch1: Flag [0b1, 8],
    cs_blank: u16 [0b11, 6],
    gain_cs3: u16 [0b11, 4],
    gain_cs2: u16 [0b1l, 2],
    gain_cs1: u16 [0b11, 0]
});

register!(
    struct GateDriveControl [0x7] {
        comm_option: CommOption [0b1, 9],
        pwm_mode: PwmMode [0b11, 7],
        dead_time: u8 [0b111, 4],
        tblank: u8 [0b11, 2],
        tvds: u8 [0b11, 0]
    }
);

register!(
    struct WarningAndWatchdog [0x1] {
        fault: Flag [0b1, 10],
        temp_flag4: Flag [0b1, 8],
        pvdd_uv: Flag [0b1, 7],
        pvdd_ov: Flag [0b1, 6],
        vds_status: Flag [0b1, 5],
        vchp_uvfl: Flag [0b1, 4],
        temp_flag1: Flag [0b1, 3],
        temp_flag2: Flag [0b1, 2],
        temp_flag3: Flag [0b1, 1],
        otw: Flag [0b1, 0]
    }
);

register!(
    struct OvVdsFaults [0x2] {
        vds_ha: Flag [0b1, 10],
        vds_la: Flag [0b1, 9],
        vds_hb: Flag [0b1, 8],
        vds_lb: Flag [0b1, 7],
        vds_hc: Flag [0b1, 6],
        vds_lc: Flag [0b1, 5],
        sns_c_ocp: Flag [0b1, 2],
        sns_b_ocp: Flag [0b1, 1],
        sns_a_ocp: Flag [0b1, 0]
    }
);

register!(
    struct IcFaults [0x3] {
        pvdd_uvlo2: Flag [0b1, 10],
        wd_fault: Flag [0b1, 9],
        otsd: Flag [0b1, 8],
        vreg_uv: Flag [0b1, 6],
        avdd_uvlo: Flag [0b1, 5],
        vcp_lsd_uvlo2: Flag [0b1, 4],
        vcph_uvlo2: Flag [0b1, 2],
        vcph_uvlo: Flag [0b1, 1],
        vcph_ovlo_abs: Flag [0b1, 0]
    }
);

register!(
    struct VgsFaults [0x4] {
        vgs_ha: Flag [0b1, 10],
        vgs_la: Flag [0b1, 9],
        vgs_hb: Flag [0b1, 8],
        vgs_lb: Flag [0b1, 7],
        vgs_hc: Flag [0b1, 6],
        vgs_lc: Flag [0b1, 5]
    }
);

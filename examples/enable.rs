#![no_main]
#![no_std]

use rtic::app;

use embedded_hal::spi::MODE_1;
use hal::gpio::{gpiob, AF6};
use hal::prelude::*;
use hal::spi::{Spi, WordSizeSixteen};
use hal::stm32;
use hal::time::Hertz;
use stm32f3xx_hal as hal;

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

use drv8305::register::Register;
use drv8305::{register::*, Drv8305};
use stm32f3xx_hal::gpio::{Output, PushPull};

type Drv = Drv8305<
    Spi<stm32::SPI3, (gpiob::PB3<AF6>, gpiob::PB4<AF6>, gpiob::PB5<AF6>), WordSizeSixteen>,
    gpiob::PB2<Output<PushPull>>,
>;

#[rtic::app(device = stm32f3xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        drv8305: Drv,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let dp: stm32::Peripherals = cx.device;

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(32.mhz())
            .sysclk(32.mhz())
            .pclk1(32.mhz())
            .pclk2(32.mhz())
            .freeze(&mut flash.acr);

        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

        // Configure pins for SPI
        let sck = gpiob.pb3.into_af6(&mut gpiob.moder, &mut gpiob.afrl);
        let miso = gpiob.pb4.into_af6(&mut gpiob.moder, &mut gpiob.afrl);
        let mosi = gpiob.pb5.into_af6(&mut gpiob.moder, &mut gpiob.afrl);

        // Chip select is active-low
        let mut ncs = gpiob
            .pb2
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        ncs.set_low();

        let mut spi = Spi::spi3(
            dp.SPI3,
            (sck, miso, mosi),
            MODE_1,
            6.mhz(),
            clocks,
            &mut rcc.apb1,
        );

        let mut drv8305 = Drv8305::new(spi, ncs);

        hprintln!("init").unwrap();

        let vds_ctrl = configure_drv(&mut drv8305);
        hprintln!("set vds: {}", vds_ctrl);

        init::LateResources { drv8305 }
    }

    #[idle(resources = [drv8305])]
    fn idle(cx: idle::Context) -> ! {
        let drv: &mut Drv = cx.resources.drv8305;

        hprintln!("idle").unwrap();

        loop {
            cortex_m::asm::delay(1_000_000);
            hprintln!("l");
            // let val = drv
            //     .exec(SpiCommand::read(Register::WarningAndWatchdog))
            //     .unwrap();
            // hprintln!("got {}", val);
        }
    }
};

fn configure_drv(drv: &mut Drv) -> u16 {
    const VDS_LEVEL: u8 = 0b00010;

    let data = (VDS_LEVEL << 3) as u16;

    let data = drv
        .modify(|gdc: GateDriveControl| {
            gdc.set_comm_option(CommOption::Active)
                .set_pwm_mode(PwmMode::One)
        })
        .unwrap();
    hprintln!("Intial val: {:?}", data.data());

    let read_data = drv.read::<GateDriveControl>().unwrap();
    hprintln!("Final val: {:?}", read_data.data());

    // let data = drv.modify(|w: GateDriveControl| {
    //     w.set_comm_option(CommOption::Active)
    //         .set_pwm_mode(PwmMode::One)
    // }).unwrap();

    //hprintln!("Set gate control to {}", )

    0
}

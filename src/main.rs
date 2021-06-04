#![no_main]
#![no_std]
#![allow(unused_labels)]

use cortex_m_rt::entry;
use stm32f1xx_hal::delay::{Delay, };
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    i2c::{DutyCycle, Mode},
    prelude::*,
    pac::{I2C1},
    timer::{Timer},
};
use ssd1306::prelude::*;
use ssd1306::{Builder};
use stm32f1xx_hal::gpio::gpiob::{PB6, PB7};
use stm32f1xx_hal::gpio::{Alternate, OpenDrain, PullUp, Input, Pxx};
use stm32f1xx_hal::i2c::BlockingI2c;
use cortex_m::peripheral::SYST;


const BOOT_DELAY_MS: u32 = 100;

mod tetris;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let (
        i2c,
        timer,
        mut p1_t1,
        mut p1_t2,
        mut p2_t1,
        mut p2_t2,
        mut p2_t3,
        mut p2_t4,
    ) = config_hardware();


    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();


    tetris::tetris(
        &mut disp, timer, &mut p1_t1, &mut p1_t2, &mut p2_t1, &mut p2_t2,
        &mut p2_t3, &mut p2_t4,
    );

    loop {}
}

fn config_hardware() -> (
    BlockingI2c<I2C1, (PB6<Alternate<OpenDrain>>, PB7<Alternate<OpenDrain>>)>,
    Timer<SYST>,
    Pxx<Input<PullUp>>,
    Pxx<Input<PullUp>>,
    Pxx<Input<PullUp>>,
    Pxx<Input<PullUp>>,
    Pxx<Input<PullUp>>,
    Pxx<Input<PullUp>>,
) {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |_cs| {
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .hclk(24.mhz())
            .sysclk(24.mhz())
            .pclk1(12.mhz())
            .pclk2(12.mhz())
            .freeze(&mut flash.acr);

        let mut delay = Delay::new(core.SYST, clocks);
        delay.delay_ms(BOOT_DELAY_MS);
        let syst = delay.free();

        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
        let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
        let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);

        let t1 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
        //let t2 = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
        let t3 = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
        let t4 = gpioa.pa2.into_pull_up_input(&mut gpioa.crl);

        let t5 = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
        //let t6 = gpioa.pa?.into_pull_up_input(&mut gpioa.crh);
        let t7 = gpioa.pa9.into_pull_up_input(&mut gpioa.crh);
        let t8 = gpiob.pb5.into_pull_up_input(&mut gpiob.crl);

        let p1_t1 = t3.downgrade();
        let p1_t2 = t5.downgrade();

        let p2_t1 = t4.downgrade();
        let p2_t2 = t8.downgrade();
        let p2_t3 = t1.downgrade();
        let p2_t4 = t7.downgrade();

        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        let i2c = BlockingI2c::i2c1(
        device.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

        let timer = Timer::syst(syst, &clocks);

        (i2c, timer, p1_t1, p1_t2, p2_t1, p2_t2, p2_t3, p2_t4)
    })
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}

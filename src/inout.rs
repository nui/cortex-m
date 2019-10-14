#![no_std]

use core::ops;
use f3::hal::{prelude::*, stm32f30x};
use f3::hal::gpio::gpioe::{self, PE1};
use f3::hal::delay::Delay;
use f3::hal::gpio::{Output, PushPull};

pub fn init() -> (Delay, gpioe::Parts) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);
    (delay, dp.GPIOE.split(&mut rcc.ahb))
}
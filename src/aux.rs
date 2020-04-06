extern crate panic_halt;

// panic handler
pub use cortex_m_rt::entry;
use stm32f4::stm32f429::{self, GPIOG, gpiok};
pub use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rng::Rng;

pub fn init() -> (Delay, GPIOG, Rng) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f429::Peripherals::take().unwrap();

    let gpiog = dp.GPIOG;

    dp.RCC.ahb1enr.write(|w| w.gpiogen().set_bit());
    dp.RCC.ahb2enr.write(|w| w.rngen().set_bit());

    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .require_pll48clk() // this is for rng
        .freeze();

    let rng = dp.RNG.constrain(clocks.clone());

    let delay = Delay::new(cp.SYST, clocks);

    (delay, gpiog, rng)
}
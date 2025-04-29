use stm32f4::stm32f429;
pub use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::prelude::*;

extern crate panic_halt; // panic handler
pub use cortex_m_rt::entry;
use stm32f4::stm32f429::GPIOG;

pub fn init() -> (Delay, GPIOG) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f429::Peripherals::take().unwrap();

    let gpiog = dp.GPIOG;

    dp.RCC.ahb1enr.write(|w| w.gpiogen().set_bit());

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let delay = Delay::new(cp.SYST, &clocks);

    (delay, gpiog)
}

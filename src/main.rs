#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32f4xx_hal::prelude::*;

use aux::entry;
use rand_core::RngCore;
use cortex_m_semihosting::hprintln;

mod aux;

#[entry]
fn main() -> ! {
    let (mut delay, gpiog, mut rng) = aux::init();

    let duration = 1 * 250 as u32;
    gpiog.moder.modify(|_, w| w.moder13().output());
    gpiog.moder.modify(|_, w| w.moder14().output());
    let off_counter = (0..3).cycle();

    for _ in 0..10 {
        let v = rng.next_u32();
        hprintln!("Random num: {}", v);
    }

    for i in off_counter {
        delay.delay_ms(duration);
        match i {
            0 => {
                gpiog.odr.modify(|_, w| w.odr13().clear_bit());
                gpiog.odr.modify(|_, w| w.odr14().clear_bit());
            }
            1 => {
                gpiog.odr.modify(|_, w| w.odr13().set_bit());
            }
            _ => {
                gpiog.odr.modify(|_, w| w.odr14().set_bit());
            }
        }
    };
    loop {}
}
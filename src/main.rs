#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{Delay, entry, Leds, prelude::*};

mod led_counter;
mod roulette;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//    for _ in led_counter::LedCounter::new(&mut leds) {
//        delay.delay_ms(75u16);
//    }

    for _ in roulette::Roulette::new(&mut leds) {
        delay.delay_ms(75u16);
    }

    loop {}
}
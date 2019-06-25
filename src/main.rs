#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

mod led_counter;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    for _ in led_counter::LedCounter::new(&mut leds) {
        delay.delay_ms(500u16);
    }

    loop {
    }
}
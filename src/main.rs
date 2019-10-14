#![no_main]
#![no_std]

use aux5::{Delay, entry, Leds, prelude::*};

mod led_counter;
mod roulette;
mod inout;

#[entry]
fn main() -> ! {
    let (mut delay, mut gpioe) = inout::init();

    let mut pe1 = gpioe.pe1.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let duration = 1 * 1000 as u32;
    loop {
        delay.delay_ms(duration);
        pe1.set_high();
        delay.delay_ms(duration);
        pe1.set_low();
    }

    loop {}
}
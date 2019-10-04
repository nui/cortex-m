use core::iter::{Cycle, Skip};
use core::ops::Range;

use aux5::Leds;
use cortex_m::asm::nop;

pub struct Roulette<'a> {
    leds: &'a mut Leds,
    is_on: bool,
    on_counter: Cycle<Range<usize>>,
    off_counter: Cycle<Range<usize>>,
}

impl<'a> Roulette<'a> {
    pub fn new(leds: &'a mut Leds) -> Self {
        for led in leds.iter_mut() {
            led.off();
        }
        let mut off_counter = (0..8).cycle();
        for _ in 0..7 {
            off_counter.next();
        }
        Self {
            leds,
            is_on: true,
            on_counter: (0..8).cycle(),
            off_counter,
        }
    }
}

impl<'a> Iterator for Roulette<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_on {
            if let Some(index) = self.on_counter.next() {
                self.leds[index].on();
            }
        } else {
            if let Some(index) = self.off_counter.next() {
                self.leds[index].off();
            }
        }
        self.is_on = !self.is_on;
        Some(())
    }
}
use aux5::{entry, prelude::*, Delay, Leds};
use core::ops::Add;

pub struct LedCounter<'a> {
    leds: &'a mut Leds,
    counter: u8,
}

impl<'a> Iterator for LedCounter<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.increment();
        Some(())
    }
}

impl<'a> LedCounter<'a> {
    pub fn increment(&mut self) {
        let now = self.counter;
        let next = match now {
            255 => 0,
            _ => now + 1,
        };
        let mut tmp = next;
        for i in 0..8 {
            let show = tmp & 0x01 == 1;
            if show {
                self.leds[i].on();
            } else {
                self.leds[i].off();
            }
            tmp >>= 1;
        }
        self.counter = next;
    }

    pub fn new(leds: &'a mut Leds) -> Self {
        for i in 0..7 {
            leds[i].off();
        }
        Self {
            leds,
            counter: 0,
        }
    }
}
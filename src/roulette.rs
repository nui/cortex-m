use aux5::Leds;

pub struct Roulette<'a> {
    leds: &'a mut Leds,
    state: u8,
}

impl<'a> Roulette<'a> {
    pub fn new(leds: &'a mut Leds) -> Self {
        for led in leds.iter_mut() {
            led.off();
        }
        Self {
            leds,
            state: Default::default(),
        }
    }
}

impl<'a> Iterator for Roulette<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.leds[0].on();
                self.leds[1].on();
            }
            1 => {
                self.leds[0].off();
            }
            2 => {
                self.leds[2].on();
            }
            3 => {
                self.leds[1].off();
            }
            4 => {
                self.leds[3].on();
            }
            5 => {
                self.leds[2].off();
            }
            6 => {
                self.leds[4].on();
            }
            7 => {
                self.leds[3].off();
            }
            8 => {
                self.leds[5].on();
            }
            9 => {
                self.leds[4].off();
            }
            10 => {
                self.leds[6].on();
            }
            11 => {
                self.leds[5].off();
            }
            12 => {
                self.leds[7].on();
            }
            13 => {
                self.leds[6].off();
            }
            14 => {
                self.leds[0].on();
            }
            _ => {
                self.leds[7].off();
            }
        }
        self.state = match self.state {
            0..=14 => self.state + 1,
            _ => 0
        };
        Some(())
    }
}
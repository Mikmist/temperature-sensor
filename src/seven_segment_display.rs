extern crate arduino_hal;

use arduino_hal::{port::{Pin, mode}, delay_us};


/// A structure representing the 7 segments of a 7-segment display
pub struct SevenSeg {
    value: u16,
    num_1: Pin<mode::Output>,
    num_2: Pin<mode::Output>,
    num_3: Pin<mode::Output>,
    num_4: Pin<mode::Output>,
    seg_a: Pin<mode::Output>,
    seg_b: Pin<mode::Output>,
    seg_c: Pin<mode::Output>,
    seg_d: Pin<mode::Output>,
    seg_e: Pin<mode::Output>,
    seg_f: Pin<mode::Output>,
    seg_g: Pin<mode::Output>,
    seg_dp: Pin<mode::Output>
}

impl SevenSeg
{
    pub fn new(
        value: u16,
        num_1: Pin<mode::Output>,
        num_2: Pin<mode::Output>,
        num_3: Pin<mode::Output>,
        num_4: Pin<mode::Output>,
        seg_a: Pin<mode::Output>,
        seg_e: Pin<mode::Output>,
        seg_c: Pin<mode::Output>,
        seg_g: Pin<mode::Output>,
        seg_b: Pin<mode::Output>,
        seg_f: Pin<mode::Output>,
        seg_d: Pin<mode::Output>,
        seg_dp: Pin<mode::Output>
    ) -> Self {
        Self {
            value,
            num_1,
            num_2,
            num_3,
            num_4,
            seg_a,
            seg_e,
            seg_c,
            seg_g,
            seg_b,
            seg_f,
            seg_d,
            seg_dp,
        }
    }

    /// Disable the 7-segment display by pulling all GPIOs low
    pub fn clear(&mut self) {
        self.seg_a(false);
        self.seg_b(false);
        self.seg_c(false);
        self.seg_d(false);
        self.seg_e(false);
        self.seg_f(false);
        self.seg_g(false);
        self.seg_dp(false)
    }

    // Enable or disable segment `a` according to the `state`
    pub fn num_1(&mut self, state: bool) {
        if state {
            self.num_1.set_high()
        } else {
            self.num_1.set_low()
        }
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn num_2(&mut self, state: bool) {
        if state {
            self.num_2.set_high()
        } else {
            self.num_2.set_low()
        }
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn num_3(&mut self, state: bool) {
        if state {
            self.num_3.set_high()
        } else {
            self.num_3.set_low()
        }
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn num_4(&mut self, state: bool) {
        if state {
            self.num_4.set_high()
        } else {
            self.num_4.set_low()
        }
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn seg_a(&mut self, state: bool) {
        if state {
            self.seg_a.set_high()
        } else {
            self.seg_a.set_low()
        }
    }

    /// Enable or disable segment `b` according to the `state`
    pub fn seg_b(&mut self, state: bool) {
        if state {
            self.seg_b.set_high()
        } else {
            self.seg_b.set_low()
        }
    }

    /// Enable or disable segment `c` according to the `state`
    pub fn seg_c(&mut self, state: bool) {
        if state {
            self.seg_c.set_high()
        } else {
            self.seg_c.set_low()
        }
    }

    /// Enable or disable segment `d` according to the `state`
    pub fn seg_d(&mut self, state: bool) {
        if state {
            self.seg_d.set_high()
        } else {
            self.seg_d.set_low()
        }
    }

    /// Enable or disable segment `e` according to the `state`
    pub fn seg_e(&mut self, state: bool) {
        if state {
            self.seg_e.set_high()
        } else {
            self.seg_e.set_low()
        }
    }

    /// Enable or disable segment `f` according to the `state`
    pub fn seg_f(&mut self, state: bool) {
        if state {
            self.seg_f.set_high()
        } else {
            self.seg_f.set_low()
        }
    }

    /// Enable or disable segment `g` according to the `state`
    pub fn seg_g(&mut self, state: bool) {
        if state {
            self.seg_g.set_high()
        } else {
            self.seg_g.set_low()
        }
    }


    /// Enable or disable segment `g` according to the `state`
    pub fn seg_dp(&mut self, state: bool) {
        if state {
            self.seg_dp.set_high()
        } else {
            self.seg_dp.set_low()
        }
    }

    /// Enable or disable segment `g` according to the `state`
    pub fn set_value(&mut self, value: u16) {
        self.value = value
    }

    /// Display the digit specified in `num`. Supported are all values in the hexadecimal system,
    /// that is `0` through `9` and `A` through `F`. Any other value will turn off the display.
    pub fn display(&mut self) {
        let mut rest = self.value;
        for i in 1..=4 {
            match i {
                4 => {
                    self.num_2(true);
                    self.num_3(true);
                    self.num_4(true);
                    self.seg_dp(false);
                    self.num_1(false);
                }
                3 => {
                    self.num_1(true);
                    self.num_3(true);
                    self.num_4(true);
                    self.seg_dp(false);
                    self.num_2(false);
                }
                2 => {
                    self.num_1(true);
                    self.num_2(true);
                    self.num_4(true);
                    self.seg_dp(true);
                    self.num_3(false);
                }
                1 => {
                    self.num_1(true);
                    self.num_2(true);
                    self.num_3(true);
                    self.seg_dp(false);
                    self.num_4(false);
                }
                _ => {}
            }
            //delay_us(100);
            match rest % 10 {
                0 => {
                    self.seg_a(true);
                    self.seg_b(true);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(false)
                }
                1 => {
                    self.seg_a(false);
                    self.seg_b(false);
                    self.seg_c(false);
                    self.seg_d(false);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(false)
                }
                2 => {
                    self.seg_a(true);
                    self.seg_b(true);
                    self.seg_c(false);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(false);
                    self.seg_g(true)
                }
                3 => {
                    self.seg_a(true);
                    self.seg_b(false);
                    self.seg_c(false);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                4 => {
                    self.seg_a(false);
                    self.seg_b(false);
                    self.seg_c(true);
                    self.seg_d(false);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                5 => {
                    self.seg_a(true);
                    self.seg_b(false);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(false);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                6 => {
                    self.seg_a(true);
                    self.seg_b(true);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(false);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                7 => {
                    self.seg_a(true);
                    self.seg_b(false);
                    self.seg_c(false);
                    self.seg_d(false);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(false)
                }
                8 => {
                    self.seg_a(true);
                    self.seg_b(true);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                9 => {
                    self.seg_a(true);
                    self.seg_b(false);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                10 => {
                    self.seg_a(false);
                    self.seg_b(true);
                    self.seg_c(true);
                    self.seg_d(true);
                    self.seg_e(true);
                    self.seg_f(true);
                    self.seg_g(true)
                }
                _ => self.clear()
            }
            rest = rest / 10;
            delay_us(2000);
            //self.clear();
            //delay_us(50);
        }
    }
}

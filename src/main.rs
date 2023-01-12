#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod millis;
mod seven_segment_display;

use hal::delay_us;
use panic_halt as _;
use dht11::Dht11;
use crate::seven_segment_display::SevenSeg;
use arduino_hal as hal;
use crate::millis::{millis, millis_init};

#[arduino_hal::entry]
unsafe fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    avr_device::interrupt::enable();

    millis_init(dp.TC0);

    let pin = pins.a1.into_opendrain();
    let mut dht11 = Dht11::new(pin);
    let mut delay = hal::Delay::new();
    let mut time_now: u32 = 0;
    // Pins for the seven segment display.
    // digitPins: 
    // segmentPins: 
    let num_1 = pins.d13.into_output().downgrade();
    let num_2 = pins.d12.into_output().downgrade();
    let num_3 = pins.d11.into_output().downgrade();
    let num_4 = pins.d10.into_output().downgrade();
    let seg_a = pins.d9.into_output().downgrade();
    let seg_b = pins.d8.into_output().downgrade();
    let seg_c = pins.d7.into_output().downgrade();
    let seg_d = pins.d6.into_output().downgrade();
    let seg_e = pins.d5.into_output().downgrade();
    let seg_f = pins.d4.into_output().downgrade();
    let seg_g = pins.d3.into_output().downgrade();
    let seg_dp = pins.d2.into_output().downgrade();
    let mut seven_segment_display = SevenSeg::new(0, num_1, num_2, num_3, num_4, seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g, seg_dp);
    
    loop {
        let time = millis();
        if time - time_now > 1000 {
            time_now = time;
            let measurement = dht11.perform_measurement(&mut delay).unwrap();
            ufmt::uwriteln!(&mut serial, "{}.{}", measurement.temperature/10, 
                measurement.temperature%10).unwrap();
            seven_segment_display.set_value(measurement.temperature as u16);
        }
        delay_us(10);
        seven_segment_display.display()
    }
}

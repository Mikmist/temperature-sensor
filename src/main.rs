#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod millis;

use panic_halt as _;
use dht11::Dht11;
use arduino_hal as hal;
use crate::millis::{millis, millis_init};

#[arduino_hal::entry]
unsafe fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    avr_device::interrupt::enable();

    millis_init(dp.TC0);

    let pin = pins.d4.into_opendrain();
    let mut led = pins.d13.into_output();
    let mut dht11 = Dht11::new(pin);
    let mut delay = hal::Delay::new();
    let mut time_now: u32 = 0;
    
    loop {
        arduino_hal::delay_ms(1000);
        let time = millis();
        if time - time_now > 1000 {
            time_now = time;
            led.toggle();
            let measurement = dht11.perform_measurement(&mut delay).unwrap();
            ufmt::uwriteln!(&mut serial, "{}.{}", measurement.temperature/10, 
                measurement.temperature%10).unwrap();
            led.toggle();
        }
    }
}

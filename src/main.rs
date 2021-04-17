#![no_std]
#![no_main]

use panic_halt as _;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use esp8266_hal::ehal::digital::v2::{InputPin, OutputPin};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let pins = dp.GPIO.split();
    let mut led = pins.gpio5.into_push_pull_output();
    let fire = pins.gpio4.into_floating_input();

    let (mut timer, _) = dp.TIMER.timers();

    led.set_low().unwrap();

    loop {
        if fire.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        timer.delay_ms(50);
    }
}

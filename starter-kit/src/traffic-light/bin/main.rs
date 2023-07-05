#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::{PB2, PD7, PD4, Dynamic}};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut red_led = pins.d10.into_output().downgrade();
    let mut yellow_led = pins.d7.into_output();
    let mut green_led = pins.d4.into_output().downgrade();

    loop {
        toggle_light(&mut green_led, &mut yellow_led);
        toggle_light(&mut red_led, &mut yellow_led);
    }
}

fn toggle_light(led: &mut Pin<Output, Dynamic>, yellow_led: &mut Pin<Output, PD7>) {
    led.set_high();
    arduino_hal::delay_ms(5000);
    led.set_low();

    for _ in 0..6 {
        yellow_led.toggle();
        arduino_hal::delay_ms(500)
    }
}
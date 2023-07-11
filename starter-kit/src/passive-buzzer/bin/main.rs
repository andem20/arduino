#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pin8 = pins.d8.into_output();

    loop {
        for _ in 0..80 {
            pin8.set_high();
            delay_ms(1);
            pin8.set_low();
            delay_ms(1);
        }

        for _ in 0..100 {
            pin8.set_high();
            delay_ms(2);
            pin8.set_low();
            delay_ms(2);
        }
    }
}

#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Timer2Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer = Timer2Pwm::new(dp.TC2, arduino_hal::simple_pwm::Prescaler::Prescale64);

    let mut led = pins.d11.into_output().into_pwm(&timer);
    led.enable();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let knob = pins.a0.into_analog_input(&mut adc);

    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        let _value = exponential(knob.analog_read(&mut adc)); // 6000 bytes

        let value = (knob.analog_read(&mut adc) >> 2) as u8; // 442 bytes

        led.set_duty(value)
    }
}

fn exponential(input: u16) -> u8 {
    let normalized = input as f32 / 1023.0;
    let rate = normalized * normalized;

    (rate * 255.0) as u8
}
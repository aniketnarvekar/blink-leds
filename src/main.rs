#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, Peripherals};
use cortex_m_rt::entry;

use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{delay::Delay, flash::FlashExt, gpio::GpioExt, pac, prelude::*, rcc::RccExt},
    switch_hal::OutputSwitch,
};

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // Create clock
    let device_peripharals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_peripharals.RCC.constrain();
    let mut flash = device_peripharals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    // Create delay
    let core_peripharals = Peripherals::take().unwrap();
    let mut delay = Delay::new(core_peripharals.SYST, clocks);

    // Create leds
    let mut gpioe = device_peripharals
        .GPIOE
        .split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let delay_ms = 100u16;

    loop {
        for led in &mut leds {
            led.on().ok();
            delay.delay_ms(delay_ms);
            led.off().ok();
            delay.delay_ms(delay_ms);
        }
    }
}

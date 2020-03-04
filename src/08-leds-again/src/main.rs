#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::{entry, prelude::*, Delay, gpioa, rcc};


#[entry]
fn main() -> ! {
    let (mut delay, gpioa, rcc) : (Delay, &'static gpioa::RegisterBlock, &'static rcc::RegisterBlock) = aux8::init();

    let us = 50000_u32;

    // enable the GPIOE peripheral
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // configure the pins as outputs
    gpioa.moder.modify(|_, w| w.moder10().output());

    loop {

        gpioa.odr.write(|w| w.odr10().set_bit());
        delay.delay_us(us);
        gpioa.odr.write(|w| w.odr10().clear_bit());
        delay.delay_us(us)
    
    }
}


#![feature(proc_macro)] // <- IMPORTANT! Feature gate for procedural macros
#![feature(const_unsafe_cell_new)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::prelude::*;
use f3::Serial;
use f3::serial::Event;
use f3::time::Hertz;
use rtfm::{app, Threshold};

const BAUD_RATE: Hertz = Hertz(115_200);

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    tasks: {
        USART1_EXTI25: {
            path: loopback,
            resources: [USART1],
        },
    }
}

// INITIALIZATION
fn init(p: init::Peripherals) {
    let serial = Serial(p.USART1);

    serial.init(BAUD_RATE.invert(), Some(p.DMA1), p.GPIOA, p.RCC);
    serial.listen(Event::Rxne);
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Send back the received byte
fn loopback(_t: &mut Threshold, r: USART1_EXTI25::Resources) {
    let serial = Serial(&**r.USART1);

    // Grab the byte we just received
    let byte = serial.read().unwrap();

    // and send it back
    serial.write(byte).unwrap();
}
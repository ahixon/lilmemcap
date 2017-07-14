#![feature(used)]
#![no_std]
 
// #[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate rk3399_m0;
extern crate rockchip;
use rockchip::serial::Serial;

extern crate embedded_hal;
use embedded_hal::serial::{Write};
 
// use cortex_m::asm;
// use cortex_m::interrupt;

fn main() {
	let uart2 = unsafe { &*rk3399_m0::UART2.get() };
	let serial = Serial(uart2);

	for byte in b"Morning sunriseeeeeee!\n" {
		let _ = serial.write(*byte);
	}
}
 
// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];
 
extern "C" fn default_handler() {
	// serial.write(b"Help! Interrupt!\n");
}
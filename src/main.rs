#![feature(used)]
#![no_std]
 
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate rk3399_m0;
 
use cortex_m::asm;
use cortex_m::interrupt;

// use basic register-flipping for now
// since we know this works
use core::ptr::{read_volatile, write_volatile};

const MMIO_BASE:u32 = 0x40000000; // on M0; on main processor it's highmem
const UART2_BASE:u32 = MMIO_BASE + 0x071A0000;

// const RBR:u32 	= 0x000;
const THR:u32 	= 0x000;
const IER:u32 	= 0x004;
const FCR:u32   = 0x008;
// const LSR:u32 	= 0x014;

pub struct Uart16650 {
	pub base: u32
}

impl Uart16650 {
	pub fn disable_interrupts(&self) {
		let interrupt_register_ptr = (self.base + IER) as *mut u32;
		let fifo_control_ptr = (self.base + FCR) as *mut u32;
		unsafe {
			write_volatile::<u32>(interrupt_register_ptr, 0);

			// disable FIFO and DMA
			write_volatile::<u32>(fifo_control_ptr, 0);
			
		}
	}

	// a direct hardware write
	// if the fifo buffer fills up, then we stop processing
	// maybe have controllable busy-wait?
	pub fn write(&self, buf: &[u8]) -> Result<usize, ()> {
		// TODO: check LSR[5] for THR Empty
		let mut total_written = 0;

		for (written, byte) in buf.iter().enumerate() {
			let data_register_ptr = (self.base + THR) as *mut u32;

			unsafe {
				// move to transmit holding register
				write_volatile::<u32>(data_register_ptr, *byte as u32);
			}

			total_written = written;
		}

		Ok(total_written)
	}
}
 
fn main() {
    let serial = Uart16650 { base: UART2_BASE }; // UART2
    // serial.disable_interrupts();
	serial.write(b"Allo from M0!\n");
}

const STACK_SIZE:usize = 0x00000200;

#[allow(dead_code)]
#[used]
#[no_mangle]
#[link_section = ".rust_stack"]
static STACK: [u32; STACK_SIZE] = [0; STACK_SIZE];
 
// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];
 
extern "C" fn default_handler() {
 //    let serial = Uart16650 { base: UART2_BASE }; // UART2
	// serial.write(b"Help! Interrupt!\n");
}
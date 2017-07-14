// use core::ptr::Unique;
use core::any::Any;
use core::ops::Deref;

extern crate rk3399_m0;
extern crate rockchip;
use rockchip::serial::Serial;

use core::fmt;

use cortex_m::interrupt;

// static STDOUT: interrupt::Mutex<Port<rk3399_m0::UART2>> = interrupt::Mutex::new(Port(Serial(&*rk3399_m0::UART2.get())));

struct Port<'a, S>(Serial<'a, S>) where S: Any + rockchip::serial::Usart;

impl<'p, S> fmt::Write for Port<'p, S> where S: Any + rockchip::serial::Usart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use hal::serial::Write;

        for byte in s.bytes() {
            loop {
                let res = self.0.write(byte);
                if res.is_ok() {
                    break;
                }
            }
        }
        Ok(())
    }
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::serial::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    interrupt::free(|cs| {
        Port(Serial(rk3399_m0::UART2.borrow(cs))).write_fmt(args).unwrap();
    });
    
}
use crate::serial::Serial;
use core::fmt;
use embedded_hal::blocking::serial::Write as _;
use riscv::interrupt;
use core::fmt::Write;

struct StdoutWrapper(Option<Serial>);

impl fmt::Write for StdoutWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(serial) = self.0.as_mut() {
            let _ = serial.bwrite_all(s.as_bytes());
        }
        Ok(())
    }
}

static mut STDOUT: StdoutWrapper = StdoutWrapper(None);

pub fn set_serial(serial: Serial) {
    interrupt::free(|_| unsafe {
        STDOUT = StdoutWrapper(Some(serial));
    })
}

/// Writes string to stdout
pub fn write_str(s: &str) {
    interrupt::free(|_| unsafe {
        let _ = STDOUT.write_str(s);
    })
}

/// Writes formatted string to stdout
pub fn write_fmt(args: fmt::Arguments) {
    interrupt::free(|_| unsafe {
        let _ = STDOUT.write_fmt(args);
    })
}

#[macro_export]
macro_rules! print {
    ($($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!($($tt)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::stdout::write_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}

#![feature(panic_info_message)]
#![no_std]
#![no_main]

use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;
use black_os::{serial_print, serial_println, QemuExitCode, exit_qemu};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 34;

/// Compares a `fmt::Arguments` instance with the `MESSAGE` string.
///
/// To use this type, write the `fmt::Arguments` instance to it using the
/// `write` macro. If a message component matches `MESSAGE`, the equals
/// field is set to true.
struct CompareMessage {
    equals: bool,
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s == MESSAGE {
            self.equals = true;
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("panic handler...");
    panic!(MESSAGE);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_message(info);
    check_location(info);

    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn check_message(info: &PanicInfo) {
    let message = info.message().unwrap_or_else(|| fail("no message"));
    let mut compare_message = CompareMessage { equals: false };
    write!(&mut compare_message, "{}", message)
        .unwrap_or_else(|_| fail("write failed"));
    if !compare_message.equals {
        fail("message not equal to expected message");
    }
}

fn check_location(info: &PanicInfo) {
    let location = info.location().unwrap_or_else(|| fail("no location"));
    if location.file() != file!() {
        fail("file name wrong");
    }
    if location.line() != PANIC_LINE {
        fail("file line wrong");
    }
}

fn fail(error: &str) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", error);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

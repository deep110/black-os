#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(black_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // rename start function in case of test

use core::panic::PanicInfo;
use black_os::println;


/// This function is called on panic.
#[cfg(not(test))] // run when not in test mode
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    black_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

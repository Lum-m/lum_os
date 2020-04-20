#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // change test main name
#![no_std] // no standard library
#![no_main] // no rust entry point
use core::{fmt::Write, panic};

mod serial;
mod vga_buffer;

#[no_mangle] // Don't mangle the name
/// The entry point
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Called on panic in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    // Not using unreachable! because this is called from a panic handler
    println!("\n\nQemu failed to exit");
    serial_println!("\n\nQemu failed to exit");

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!();
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial_assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo};


#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "C" fn _start() {
    // Set the stack pointer
    asm!("ldr x0, =0x8004000", "mov sp, x0");
    loop{}
}

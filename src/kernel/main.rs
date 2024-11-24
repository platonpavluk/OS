#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
mod vga_buffer;
mod keyboard;
mod commands;
use commands::process_commands;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::clear_screen();
    vga_buffer::println("Welcome to RUXS!");
    vga_buffer::println("Type 'help' to see available commands.");

    process_commands();

    loop {}
}


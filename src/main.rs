#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer; // Подключаем модуль с буфером VGA
mod command; // Подключаем модуль с обработкой команд

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Заглушка для имитации чтения ввода
fn read_command() -> &'static str {
    "hello" // Для примера, всегда возвращаем "hello"
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something(); // Выводим подсказку
    
    // Читаем команду
    let command = read_command();
    
    // Обрабатываем команду через модуль commands
    command::process_command(command);
    
    loop {}
}

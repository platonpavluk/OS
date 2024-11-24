// src/commands.rs
use crate::keyboard;
use crate::vga_buffer;
use crate::vga_buffer::print;
use core::arch::asm;
use core::ptr;



/// Обробляє команди, зчитуючи з клавіатури
/// Обробляє команди, зчитуючи з клавіатури та тримає курсор біля підказки
pub fn process_commands() {
    let mut buffer = [0u8; 1024]; // Буфер для введених даних
    let mut index = 0;
    vga_buffer::print("> "); 

    loop {
        if let Some(byte) = keyboard::read_key() {
            match byte {
                b'\n' => {
                    let command = core::str::from_utf8(&buffer[..index]).unwrap_or("");
                    // Натискання Enter: обробка команди
                    if index > 0 {
                        
                        execute_command(command);
                        index = 0; // Очищаємо буфер
                    }
                    
                    vga_buffer::println(""); // Перехід на новий рядок після команди
                    print("> ");
                }
                8 => {
                    // Backspace: видалення символу
                    if index > 0 {
                        index -= 1;
                        vga_buffer::backspace(); // Видаляє символ на екрані
                    }
                }
                _ => {
                    // Додавання символу в буфер
                    if index < buffer.len() {
                        buffer[index] = byte;
                        index += 1;
                        vga_buffer::write_char(byte as char); // Виводимо символ біля підказки
                    }
                }
            }
        }
    }
}

fn execute_command(cmd: &str) {
    match cmd {
        "help" => {
            vga_buffer::println("\nAvailable commands:");
            vga_buffer::println("\nhelp    - Show this help message");
            vga_buffer::println("\ncl   - Clear the screen");
        }
        "rb" => {
            reboot_system()
        }

        "cl" => {
            vga_buffer::clear_screen();
        }
        _ => {
            vga_buffer::println("\nUnknown command. Type 'help' for a list of commands.");
        }
    }
}

fn reboot_system() {
    vga_buffer::println("\nRebooting system...");
    // Перезавантаження системи (це може бути замкнення на асемблері, перезапуск у вашій тестовій середовищі)
    unsafe {
        asm!("int3"); // Викликає програму для перезавантаження
    }
}

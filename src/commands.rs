// src/commands.rs
use crate::keyboard;
use crate::vga_buffer;
use crate::vga_buffer::print;



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
                    // Натискання Enter: обробка команди
                    if index > 0 {
                        let command = core::str::from_utf8(&buffer[..index]).unwrap_or("");
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
            vga_buffer::println("\nreboot  - Reboot the system (stub)");
            vga_buffer::println("\nshutdown - Shutdown the system (stub)");
        }
        "cl" => {
            vga_buffer::clear_screen();
        }
        "reboot" => {
            vga_buffer::println("\nRebooting... (not implemented)");
        }
        "shutdown" => {
            vga_buffer::println("\nShutting down... (not implemented)");
        }
        _ => {
            vga_buffer::println("\nUnknown command. Type 'help' for a list of commands.");
        }
    }
}
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const CURSOR_CMD_PORT: u16 = 0x3D4;
const CURSOR_DATA_PORT: u16 = 0x3D5;

use core::ptr::write_volatile;

pub fn clear_screen() {
    for i in 0..(80 * 25) {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = b' ';
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0x07;
        }
        clear_input_history()
    }
}

pub fn write_char(c: char) {
    static mut COLUMN: usize = 0;
    static mut ROW: usize = 0;

    if c == '\n' {
        unsafe {
            ROW += 1;
            COLUMN = 0;
        }
        return;
    }

    unsafe {
        let pos = ROW * 80 + COLUMN;
        *VGA_BUFFER.offset(pos as isize * 2) = c as u8;
        *VGA_BUFFER.offset(pos as isize * 2 + 1) = 0x07;

        COLUMN += 1;
        if COLUMN >= 80 {
            ROW += 1;
            COLUMN = 0;
        }
        if ROW >= 25 {
            clear_screen();
            ROW = 0;
        }
    }
}

pub fn backspace() {
    static mut COLUMN: usize = 0;
    static mut ROW: usize = 0;

    unsafe {
        if COLUMN > 0 {
            COLUMN -= 1;
        } else if ROW > 0 {
            ROW -= 1;
            COLUMN = 79;
        }
        let pos = ROW * 80 + COLUMN;
        *VGA_BUFFER.offset(pos as isize * 2) = b' ';
    }
}

pub fn println(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
    write_char('\n');
}

pub fn print(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}



pub fn clear_input_history() {
    // Припускаємо, що введення зберігається в масиві `buffer`
    let mut buffer = [0u8; 1024]; // Масив для введення

    // Очищаємо всі старі символи та '\n'
    for i in 0..buffer.len() {
        buffer[i] = 0;
    }
}
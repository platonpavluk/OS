use x86_64::instructions::port::Port;


static mut LAST_SCANCODE: u8 = 0;

pub fn read_key() -> Option<u8> {
    let mut port: Port<u8> = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    unsafe {
        if scancode == LAST_SCANCODE {
            return None;
        }
        LAST_SCANCODE = scancode;
    }

    if scancode >= 0x80 {
        return None;
    }

    scancode_to_ascii(scancode)
}

fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02 => Some(b'1'),
    0x03 => Some(b'2'),
    0x04 => Some(b'3'),
    0x05 => Some(b'4'),
    0x06 => Some(b'5'),
    0x07 => Some(b'6'),
    0x08 => Some(b'7'),
    0x09 => Some(b'8'),
    0x0A => Some(b'9'),
    0x0B => Some(b'0'),
    0x1C => Some(b'\n'),  // Enter
    0x0E => Some(8),      // Backspace
    0x10 => Some(b'q'),   // q
    0x11 => Some(b'w'),   // w
    0x12 => Some(b'e'),   // e
    0x13 => Some(b'r'),   // r
    0x14 => Some(b't'),   // t
    0x15 => Some(b'y'),   // y
    0x16 => Some(b'u'),   // u
    0x17 => Some(b'i'),   // i
    0x18 => Some(b'o'),   // o
    0x19 => Some(b'p'),   // p
    0x1E => Some(b'a'),   // a
    0x1F => Some(b's'),   // s
    0x20 => Some(b'd'),   // d
    0x21 => Some(b'f'),   // f
    0x22 => Some(b'g'),   // g
    0x23 => Some(b'h'),   // h
    0x24 => Some(b'j'),   // j
    0x25 => Some(b'k'),   // k
    0x26 => Some(b'l'),   // l
    0x2C => Some(b'z'),   // z
    0x2D => Some(b'x'),   // x
    0x2E => Some(b'c'),   // c
    0x2F => Some(b'v'),   // v
    0x30 => Some(b'b'),   // b
    0x31 => Some(b'n'),   // n
    0x32 => Some(b'm'),   // m
    0x33 => Some(b','),   // ,
    0x34 => Some(b'.'),   // .
    0x35 => Some(b'/'),   // /
    0x39 => Some(b' '),   // Space
        _ => None, 
    }
}

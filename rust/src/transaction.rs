//! Implements the serial command handler

use crate::ws2812b::{Matrix, Rgb};
use alloc::vec::Vec;
use core::str;
use picolib::{error::Error, sys};

/// The valid chars
const VALID_CHARS: &[u8] = b"0123456789: \n";

/// Reads a line from stdin
fn read_line(buf: &mut Vec<u8>) -> Result<(), Error> {
    // Read from stdin
    'read_loop: loop {
        // Read the next byte and ignore invalid ones
        let mut byte = 0;
        if unsafe { sys::pico_stdio_getc_timeout(&mut byte, 7 * 1000) } != 0 {
            continue 'read_loop;
        }
        if !VALID_CHARS.contains(&byte) {
            continue 'read_loop;
        }

        // Validate abort conditions
        if byte == b'\n' {
            return Ok(());
        }
        if buf.len() == buf.capacity() {
            return Err(error!("Command is too large"));
        }
        buf.push(byte);
    }
}

/// Parses a command, returns `(strip, led, rgb_values)`
fn parse_command(field: &str) -> Result<(usize, usize, Rgb), Error> {
    // Split the field and take the segments
    let mut segments = field.split(':');
    let strip = segments.next().ok_or(error!("Field is truncated"))?;
    let led = segments.next().ok_or(error!("Field is truncated"))?;
    let red = segments.next().ok_or(error!("Field is truncated"))?;
    let green = segments.next().ok_or(error!("Field is truncated"))?;
    let blue = segments.next().ok_or(error!("Field is truncated"))?;

    if segments.next().is_some() {
        return Err(error!("Field is too long"));
    }

    // Parse the segments
    let strip: usize = strip.parse().map_err(|_| error!("Strip index is invalid"))?;
    let led: usize = led.parse().map_err(|_| error!("LED index is invalid"))?;
    let red: u8 = red.parse().map_err(|_| error!("RGB value (red) is invalid"))?;
    let green: u8 = green.parse().map_err(|_| error!("RGB value (green) is invalid"))?;
    let blue: u8 = blue.parse().map_err(|_| error!("RGB value (blue) is invalid"))?;

    let rgb = Rgb::new(red, green, blue);
    Ok((strip, led, rgb))
}

/// Reads a command into `linebuf`, parses it and applies the changes to `state`
pub fn parse<'a>(linebuf: &'a mut Vec<u8>, state: &mut [Matrix]) -> Result<&'a str, Error> {
    // Read a line
    linebuf.clear();
    read_line(linebuf)?;
    let line = str::from_utf8(linebuf).map_err(|_| error!("Non-UTF-8 char in line?!"))?;

    // Get the transaction ID
    let mut fields = line.split(' ');
    let transaction_id = fields.next().ok_or(error!("Truncated command ID"))?;

    // Split the line into fields
    for field in fields {
        // Parses the field and update the appropriate strip
        let (strip, led, rgb) = parse_command(field)?;
        let strip = state.get_mut(strip).ok_or(error!("Strip index is invalid"))?;
        strip.set(led, rgb)?;
    }
    Ok(transaction_id)
}

use std::fs::File;
use std::io::{BufReader, Read};
use std::str::from_utf8;
use std::vec;

fn bcd_to_f64(bytes: Vec<u8>) -> Option<f64> {
    let mut value = 0.0;
    let mut half_bytes = vec![];

    for digit in bytes {
        half_bytes.push((digit >> 4) & 0x0F);
        half_bytes.push(digit & 0x0F);
    }

    for half_byte in half_bytes {
        if half_byte <= 0x09 {
            value = value * 10.0 + half_byte as f64;
        }

        if half_byte == 0x0B {
            return None;
        }
        if half_byte == 0x0C && value < 0.0 {
            value *= -1.0;
        }
        if half_byte == 0x0D && value >= 0.0 {
            value *= -1.0;
        }
        if half_byte == 0x0E {
            break;
        }
    }

    Some(value)
}

pub fn read_bytes(reader: &mut BufReader<File>, size: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer).unwrap();

    buffer
}

pub fn read_u8(reader: &mut BufReader<File>) -> u8 {
    let bytes = read_bytes(reader, 1);

    u8::from_be_bytes(bytes[0..1].try_into().unwrap())
}

pub fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let bytes = read_bytes(reader, 2);

    u16::from_be_bytes(bytes[0..2].try_into().unwrap())
}

pub fn read_u32(reader: &mut BufReader<File>) -> u32 {
    let bytes = read_bytes(reader, 4);

    u32::from_be_bytes(bytes[0..4].try_into().unwrap())
}

pub fn read_ascii(reader: &mut BufReader<File>, size: usize) -> String {
    let bytes = read_bytes(reader, size);

    from_utf8(&bytes).unwrap().to_string()
}

fn read_lonlat(reader: &mut BufReader<File>) -> f64 {
    let bytes = read_bytes(reader, 4);
    let value = bcd_to_f64(bytes).unwrap();

    value / 10000.0
}

pub fn read_longitude(reader: &mut BufReader<File>) -> f64 {
    let value = read_lonlat(reader);

    if value > 180.0 {
        value - 360.0
    } else {
        value
    }
}

pub fn read_latitude(reader: &mut BufReader<File>) -> f64 {
    let value = read_lonlat(reader);

    if value > 90.0 {
        value - 180.0
    } else {
        value
    }
}

pub fn read_altitude(reader: &mut BufReader<File>) -> f64 {
    let bytes = read_bytes(reader, 4);
    let value = bcd_to_f64(bytes).unwrap();

    value / 10.0
}

extern crate adler32;

use adler32::RollingAdler32;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 0x1000;
const KEY_SIZE: usize = 0x400;
const MASTER_KEY: [u8; KEY_SIZE] = [
    0x15, 0xFE, 0x43, 0x67, 0xAF, 0xB3, 0xCF, 0xC6, 0x08, 0x8B, 0xA3, 0x2A, 0x60, 0x4C, 0x60, 0x7A,
    0x5F, 0x54, 0x3E, 0x72, 0xD5, 0xA9, 0x37, 0x3D, 0x34, 0x04, 0xA9, 0x8D, 0x7C, 0x8A, 0xC6, 0x0F,
    0x80, 0x6A, 0x5F, 0xEC, 0xE0, 0xC0, 0xCE, 0x6E, 0xD1, 0xD5, 0x91, 0x2B, 0x28, 0x38, 0xA2, 0x53,
    0x3A, 0x11, 0x3A, 0x32, 0x75, 0x3C, 0xB6, 0x6E, 0x95, 0xD3, 0x3E, 0xA1, 0xA4, 0xD9, 0xBE, 0x6F,
    0x63, 0x10, 0xAD, 0x68, 0xB3, 0x4A, 0x7C, 0xC6, 0x46, 0xBB, 0x36, 0xBD, 0x4E, 0x70, 0x37, 0x24,
    0x57, 0x3D, 0xE7, 0x9D, 0x4B, 0xF7, 0x3E, 0xDF, 0x06, 0x05, 0x2E, 0xA8, 0x26, 0xBA, 0x84, 0xFC,
    0x1F, 0x9C, 0x8A, 0x5E, 0xF7, 0xB1, 0x49, 0x62, 0x9E, 0xCF, 0x66, 0x4F, 0x9D, 0xBF, 0x53, 0xD3,
    0x03, 0x1F, 0x22, 0x87, 0x2C, 0xC8, 0xDD, 0xAD, 0xF0, 0x2B, 0xB4, 0xF1, 0xB2, 0xE3, 0x04, 0x1C,
    0x0E, 0xFB, 0x75, 0x20, 0xC9, 0x51, 0x79, 0xC6, 0x4B, 0x4D, 0xE3, 0x04, 0x76, 0xC4, 0x9B, 0x1D,
    0xF1, 0x11, 0x78, 0x7D, 0x21, 0xEC, 0x45, 0x8B, 0xE2, 0xF9, 0x9E, 0x1C, 0xE0, 0x1F, 0x6F, 0x53,
    0xCB, 0x55, 0x52, 0xE0, 0x84, 0xA9, 0xF7, 0xDE, 0x4F, 0xE1, 0x61, 0x03, 0xEC, 0x1B, 0x4D, 0xCF,
    0xB8, 0x95, 0x0D, 0xB6, 0xCC, 0x03, 0x92, 0x59, 0xF8, 0x9B, 0x97, 0xC7, 0x62, 0x23, 0xAE, 0x90,
    0xFD, 0x28, 0x48, 0x0B, 0x56, 0x70, 0xE6, 0xE1, 0x7B, 0x7E, 0x2E, 0xEF, 0x74, 0xBD, 0xA3, 0xA2,
    0xFA, 0x4B, 0xE3, 0x40, 0xE9, 0x3B, 0x86, 0xED, 0xF3, 0xE2, 0xE3, 0x96, 0xA7, 0xB9, 0xBD, 0x8E,
    0x5A, 0xD6, 0x5B, 0x10, 0x85, 0xF7, 0x1D, 0xA8, 0x00, 0x39, 0xC0, 0x2D, 0xE8, 0xB6, 0xEF, 0xD9,
    0x28, 0xAA, 0x64, 0xF6, 0xAB, 0xBF, 0x6F, 0xDB, 0x48, 0x16, 0xAC, 0x76, 0xAA, 0x5E, 0xB8, 0xAD,
    0x89, 0x37, 0x0D, 0x4D, 0x43, 0x4C, 0x03, 0xF1, 0xC0, 0xD7, 0x44, 0x96, 0x2B, 0xDF, 0xEA, 0x50,
    0x6D, 0xC3, 0xA7, 0xFB, 0x85, 0x66, 0x0F, 0x78, 0x3B, 0x99, 0x73, 0xEF, 0xB1, 0x3E, 0x4A, 0x06,
    0xC2, 0x07, 0xA9, 0xD4, 0x71, 0x34, 0xD6, 0x3A, 0xF6, 0x9F, 0x99, 0xFB, 0xAC, 0xAA, 0x2E, 0x07,
    0x19, 0xF8, 0xD1, 0xA5, 0xEB, 0x8A, 0x96, 0x73, 0x6C, 0x24, 0xB4, 0x67, 0xA0, 0xA7, 0x92, 0xEB,
    0x42, 0x0A, 0x38, 0x47, 0x5E, 0x5D, 0xC2, 0xDA, 0x4E, 0xA8, 0x19, 0xAC, 0xB3, 0xB1, 0xF4, 0x4F,
    0x0E, 0x9D, 0xD2, 0xB8, 0x43, 0x6D, 0x97, 0xFE, 0x20, 0x8C, 0x58, 0xAE, 0x65, 0x6C, 0xD0, 0x76,
    0x13, 0x90, 0xC5, 0xB4, 0xB0, 0xEC, 0x54, 0x0E, 0x08, 0xCA, 0x09, 0xFA, 0x8D, 0x86, 0x3C, 0x46,
    0x12, 0xAB, 0x6B, 0x3B, 0x85, 0x34, 0x89, 0x87, 0x14, 0xA0, 0x57, 0xEB, 0xAF, 0xBA, 0xB7, 0x75,
    0x09, 0xB9, 0x5A, 0x4B, 0x13, 0xF9, 0x7F, 0xEF, 0x5C, 0xBC, 0xC5, 0x7E, 0xDB, 0xE7, 0x72, 0x17,
    0x70, 0x0C, 0x44, 0xCB, 0xD8, 0x26, 0x55, 0xAA, 0x36, 0xC5, 0x8E, 0x9A, 0x47, 0xBB, 0xDC, 0x0B,
    0x2C, 0x30, 0x52, 0x93, 0x93, 0x17, 0x7E, 0xDC, 0x46, 0xF5, 0xFA, 0xA4, 0xE4, 0x75, 0xE7, 0x5E,
    0x63, 0x35, 0x48, 0x31, 0x5B, 0xD2, 0x8C, 0x18, 0xD8, 0x07, 0x25, 0x33, 0x7B, 0x60, 0xB2, 0x78,
    0x7D, 0x81, 0xDA, 0xD0, 0xB7, 0x88, 0xEA, 0x95, 0x48, 0xD6, 0xF8, 0xED, 0x61, 0xC8, 0xD8, 0x80,
    0x9C, 0xA4, 0x67, 0x5A, 0x98, 0x14, 0x5B, 0x69, 0x2D, 0x18, 0x20, 0x9D, 0x64, 0x74, 0x14, 0x11,
    0x56, 0x9A, 0x77, 0xB5, 0xAB, 0x64, 0x59, 0x06, 0xF2, 0x17, 0xB5, 0x20, 0xB4, 0xCE, 0xD9, 0x72,
    0x99, 0xED, 0xB5, 0x87, 0xAE, 0x3F, 0x27, 0x59, 0xB5, 0xE7, 0xE0, 0x22, 0xC9, 0x35, 0xF8, 0x7C,
    0x24, 0xC9, 0x54, 0xDB, 0x5C, 0x80, 0x67, 0x29, 0x97, 0xD0, 0x5D, 0x9E, 0x0B, 0x68, 0x82, 0xAF,
    0xF4, 0xE5, 0xC4, 0x7E, 0xBE, 0x02, 0xDC, 0xF2, 0x21, 0xE4, 0xF0, 0x08, 0x0D, 0xFF, 0x56, 0x74,
    0x82, 0x6B, 0x8B, 0x79, 0x29, 0x12, 0x38, 0x3D, 0xEE, 0x1E, 0x66, 0xDD, 0x45, 0x6B, 0xBF, 0xDD,
    0x1C, 0x4F, 0xD9, 0x08, 0x76, 0x2F, 0x52, 0x6A, 0x17, 0x2F, 0xBC, 0x13, 0x1A, 0x37, 0xDE, 0xDD,
    0x5C, 0x1F, 0x31, 0x0F, 0x16, 0xF2, 0xFE, 0x83, 0x8F, 0xF3, 0xBB, 0x0C, 0x15, 0x8A, 0x2A, 0x91,
    0x30, 0x2A, 0x4A, 0x7D, 0x53, 0x8D, 0x35, 0xAF, 0x15, 0x86, 0x1C, 0x27, 0xEE, 0x33, 0x1B, 0x66,
    0x2D, 0x41, 0x10, 0x91, 0x02, 0xD7, 0x4D, 0xA6, 0xC8, 0x5F, 0xD4, 0x72, 0x4E, 0x1B, 0x71, 0xCA,
    0x8C, 0xE9, 0x1D, 0xC1, 0x43, 0x44, 0x90, 0x82, 0xCA, 0x11, 0xEE, 0x30, 0x8B, 0x6E, 0xB3, 0xC2,
    0xD8, 0x9E, 0xCC, 0x04, 0x50, 0x86, 0xCF, 0x1D, 0xB1, 0x2B, 0x3A, 0x34, 0x78, 0x1A, 0x3B, 0xA0,
    0xE1, 0xE6, 0x15, 0x29, 0xCC, 0x27, 0x5A, 0x3C, 0xCB, 0x4E, 0x0D, 0x4C, 0x00, 0x98, 0x01, 0xAC,
    0x6C, 0xCD, 0x84, 0x65, 0xB9, 0x51, 0xD1, 0xAE, 0xCC, 0x62, 0x28, 0xA2, 0x89, 0xF4, 0xC3, 0x61,
    0x27, 0xF5, 0x7F, 0xEA, 0xCA, 0x1E, 0x84, 0xE5, 0x2C, 0x12, 0xCE, 0xA5, 0x7A, 0xC4, 0x40, 0x02,
    0xC1, 0xCD, 0xA3, 0x8F, 0x38, 0x7F, 0x51, 0x39, 0xE8, 0x21, 0x1B, 0x39, 0x9F, 0xA2, 0x69, 0x7D,
    0x69, 0x94, 0xE2, 0xDB, 0xC2, 0xD4, 0x55, 0xB7, 0xEB, 0x61, 0xD3, 0xE5, 0x73, 0xFC, 0xC8, 0x98,
    0xC7, 0x3F, 0x71, 0xFF, 0x59, 0x47, 0x68, 0x42, 0x1E, 0xFD, 0x9C, 0x26, 0x00, 0xA6, 0xB2, 0xFC,
    0xE6, 0x5D, 0xD0, 0xA8, 0x05, 0x69, 0xA5, 0x09, 0x16, 0xA4, 0x31, 0xB6, 0x0C, 0xDC, 0x93, 0xC4,
    0x71, 0xD7, 0x40, 0x91, 0x97, 0x01, 0xB9, 0x10, 0x0E, 0xDA, 0x88, 0xE9, 0x22, 0x5F, 0x9B, 0x60,
    0x23, 0x05, 0xB0, 0xD2, 0x3F, 0x81, 0x13, 0x64, 0x58, 0xDF, 0xEE, 0xC7, 0xA3, 0x80, 0xBE, 0x40,
    0xA5, 0x6C, 0x9B, 0x5D, 0x0A, 0xB2, 0x92, 0x88, 0x6D, 0xC9, 0xE6, 0x83, 0x49, 0xF9, 0x3C, 0x01,
    0x33, 0xEA, 0x5C, 0x49, 0xFF, 0x50, 0xAB, 0x87, 0x0A, 0x21, 0x74, 0xFD, 0xF3, 0x44, 0xF5, 0xBB,
    0x22, 0x07, 0x57, 0x89, 0xED, 0x7A, 0x8E, 0x9A, 0xFE, 0x94, 0x8D, 0x01, 0x6D, 0x23, 0xC5, 0x8C,
    0x81, 0x29, 0xE8, 0x8E, 0x58, 0x83, 0xF6, 0x77, 0x1E, 0x16, 0xD5, 0x95, 0xF9, 0x83, 0x46, 0x57,
    0xD1, 0xA1, 0x65, 0x50, 0xDE, 0xDF, 0x49, 0x9C, 0x32, 0x05, 0xA6, 0xA6, 0x99, 0x8F, 0x06, 0xE4,
    0x2F, 0x3F, 0xE5, 0x2D, 0xBD, 0x25, 0xF0, 0xC1, 0x56, 0xC3, 0x3D, 0x9F, 0xB0, 0x32, 0x30, 0x9F,
    0x2F, 0xF1, 0xE9, 0x92, 0x7B, 0x94, 0xBE, 0x31, 0xF3, 0x0C, 0xBA, 0x02, 0x14, 0x81, 0xCE, 0x77,
    0xF4, 0x0B, 0xCB, 0x35, 0xD7, 0x62, 0x51, 0x79, 0x94, 0x8F, 0x90, 0xF0, 0x7C, 0x42, 0xCD, 0x68,
    0x4C, 0xA1, 0x41, 0xFF, 0x1A, 0x1A, 0x63, 0xB0, 0x5B, 0x52, 0x2E, 0x41, 0x23, 0x19, 0x26, 0x6B,
    0x6A, 0xF5, 0x42, 0xD6, 0x55, 0xDA, 0xBF, 0xCD, 0xE2, 0x25, 0x88, 0xC3, 0x33, 0xFB, 0xF2, 0xBC,
    0x63, 0xA0, 0x36, 0xF6, 0xD2, 0x93, 0x73, 0x2C, 0xA1, 0x7B, 0x70, 0x45, 0x6E, 0xB8, 0x0A, 0xE8,
    0xB7, 0x7A, 0xD4, 0x98, 0xE4, 0x24, 0x18, 0x32, 0x58, 0x45, 0x47, 0x7F, 0x85, 0xD5, 0x4A, 0xC7,
    0xAD, 0xE1, 0x77, 0xBC, 0x54, 0x41, 0x12, 0x2A, 0x00, 0xC1, 0x82, 0xFD, 0xFC, 0x25, 0x96, 0x6A,
    0x6F, 0x18, 0x79, 0x39, 0xDE, 0x36, 0xBA, 0xD3, 0x19, 0x9A, 0x65, 0xA7, 0x09, 0xFA, 0x0F, 0xC0,
];

fn get_output_filename(input_filename: &str) -> Result<String, Box<dyn Error>> {
    let filename = match input_filename.rsplit_once('.') {
        Some((filename, extension)) => {
            let extension = {
                let lc_extension = extension.to_lowercase();
                if lc_extension == "aud" {
                    Some("mp3")
                } else if lc_extension == "pnz" {
                    Some("png")
                } else {
                    None
                }
            };
            match extension {
                Some(extension) => Some(format!("{}.{}", filename, extension)),
                _ => None,
            }
        }
        _ => None,
    };
    match filename {
        Some(filename) => Ok(filename),
        _ => Err("Couldn't determine the output filename".into()),
    }
}

fn decrypt(input_filename: &str) -> Result<(), Box<dyn Error>> {
    let output_filename = get_output_filename(&input_filename)?;
    let mut input = File::open(input_filename)?;
    let mut current_adler32 = RollingAdler32::new();
    let expected_adler32 = {
        let mut buffer = [0; 4];
        input.read_exact(&mut buffer)?;
        u32::from_le_bytes(buffer)
    };
    let mut read_buffer = Vec::from([0; BUFFER_SIZE]);
    let mut out_buffer = Vec::with_capacity(BUFFER_SIZE);
    let mut key_offset = expected_adler32 as usize % KEY_SIZE;
    loop {
        let data_end = input.read(&mut read_buffer[..])?;
        if data_end == 0 {
            break;
        }
        for byte in &mut read_buffer[..data_end] {
            *byte ^= MASTER_KEY[key_offset];
            *byte = byte.reverse_bits();
            key_offset = (key_offset + 1) % KEY_SIZE;
        }
        out_buffer.reserve(data_end);
        out_buffer.extend_from_slice(&mut read_buffer[..data_end]);
        current_adler32.update_buffer(&read_buffer[..data_end]);
        if data_end < BUFFER_SIZE {
            break;
        }
    }
    if current_adler32.hash() == expected_adler32 {
        let mut output = File::create(output_filename)?;
        output.write_all(&out_buffer)?;
        Ok(())
    } else {
        Err(format!(
            "Adler32 mismatch: Expected {:#010X}, got {:#010X}",
            expected_adler32,
            current_adler32.hash()
        )
        .into())
    }
}

fn main() {
    let mut show_help = true;
    let mut has_errors = false;

    for file in env::args().skip(1) {
        decrypt(&file).unwrap_or_else(|err| {
            has_errors = true;
            eprintln!("Error decrypting {}: {}", file, err);
        });
        show_help = false;
    }

    if show_help {
        eprintln!("Usage: apdec_legacy FILE1.AUD [FILE2.PNZ] [FILE3.AUD] [...]");
    }

    if show_help || has_errors {
        std::process::exit(1);
    }
}

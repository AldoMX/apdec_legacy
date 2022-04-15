extern crate adler32;
extern crate rayon;

use adler32::RollingAdler32;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

mod constants;
use constants::{BUFFER_SIZE, U32_SIZE};

mod keys;
use keys::KEY_PIU_EXTRA;

mod types;
use types::Result;

fn get_decrypted_buffer(file: &File, key: &[u8]) -> Result<Vec<u8>> {
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut buffer = reader.fill_buf()?;
    let mut buffer_len = buffer.len();
    if buffer_len < U32_SIZE {
        return Err("Couldn't determine the expected Adler32".into());
    }

    let mut current_adler32 = RollingAdler32::new();
    let expected_adler32 = {
        let mut adler32_buffer = [0; U32_SIZE];
        adler32_buffer.copy_from_slice(&buffer[..U32_SIZE]);
        u32::from_le_bytes(adler32_buffer)
    };
    let mut out_buffer = Vec::with_capacity(buffer.len() - U32_SIZE);
    let mut data_start = U32_SIZE;
    let mut key_offset = expected_adler32 as usize % key.len();

    loop {
        for byte in &buffer[data_start..] {
            let out_byte = (byte ^ key[key_offset]).reverse_bits();
            current_adler32.update(out_byte);
            out_buffer.push(out_byte);
            key_offset += 1;
            key_offset %= key.len();
        }
        reader.consume(buffer_len);
        if buffer_len < BUFFER_SIZE {
            break;
        }
        buffer = reader.fill_buf()?;
        buffer_len = buffer.len();
        if buffer_len == 0 {
            break;
        }
        data_start = 0;
        out_buffer.reserve(buffer_len);
    }

    if current_adler32.hash() == expected_adler32 {
        Ok(out_buffer)
    } else {
        Err(format!(
            "Adler32 mismatch: Expected {:#010X}, got {:#010X}",
            expected_adler32,
            current_adler32.hash()
        )
        .into())
    }
}

fn get_output_filename(input_filename: &str) -> Option<String> {
    match input_filename.rsplit_once('.') {
        Some((filename, extension)) => match extension.to_lowercase().as_str() {
            "aud" => Some([filename, ".mp3"].concat()),
            "pnz" => Some([filename, ".png"].concat()),
            _ => None,
        },
        _ => None,
    }
}

fn decrypt(input_filename: &str) -> Result<()> {
    let output_filename =
        get_output_filename(&input_filename).ok_or("Couldn't determine the output filename")?;
    let input_file = File::open(input_filename)?;
    let decrypted_buffer = get_decrypted_buffer(&input_file, &KEY_PIU_EXTRA)?;
    let mut output = File::create(output_filename)?;
    output.write_all(&decrypted_buffer)?;
    Ok(())
}

fn main() {
    let files: Vec<String> = std::env::args().skip(1).collect();
    if files.len() == 0 {
        eprintln!("Usage: apdec_legacy FILE1.AUD [FILE2.PNZ] [FILE3.AUD] [...]");
        std::process::exit(1);
    }

    let num_errors = files
        .into_par_iter()
        .filter(|file| {
            if let Err(error) = decrypt(&file) {
                eprintln!("Error decrypting {}: {}", file, error);
                true
            } else {
                eprintln!("Success decrypting {}", file);
                false
            }
        })
        .count();

    if num_errors > 0 {
        std::process::exit(1);
    }
}

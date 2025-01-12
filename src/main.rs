use colored::Colorize;
use std::fs::File;
use std::io::{self, Read};
use std::str;

/// Turn a byte into its colored,
/// 2-digit hex representation
fn color_hex(byte: u8) -> String {
    // format a byte into a 2-digit hex string
    // https://doc.rust-lang.org/std/fmt/#sign
    let formatted_byte = format!("{:02x}", byte);

    // https://www.marcusfolkesson.se/til/xxd-color-support
    //
    // https://github.com/vim/vim/blob/master/src/xxd/xxd.c
    // -- snip --
    //       #if defined(__MVS__) && __CHARSET_LIB == 0
    //       if (e >= 64)
    //         l[(*c)++] = COLOR_GREEN;
    //       #else
    //       if (e > 31 && e < 127)
    //         l[(*c)++] = COLOR_GREEN;
    //       #endif
    //
    //       else if (e == 9 || e == 10 || e == 13)
    //         l[(*c)++] = COLOR_YELLOW;
    //       else if (e == 0)
    //         l[(*c)++] = COLOR_WHITE;
    //       else if (e == 255)
    //         l[(*c)++] = COLOR_BLUE;
    //       else
    //         l[(*c)++] = COLOR_RED;
    // -- snip --
    match byte {
        31..127 => formatted_byte.green().to_string(),
        9 | 10 | 13 => formatted_byte.yellow().to_string(),
        0 => formatted_byte.white().to_string(),
        255 => formatted_byte.blue().to_string(),
        _ => formatted_byte.red().to_string(),
    }
}

fn main() -> io::Result<()> {
    let mut file = match File::open("light_brigade.txt") {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let mut buffer = [0u8; 16];
    let mut offset: usize = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        offset += buffer.len();

        let mut hex = String::new();

        for (i, byte) in buffer.iter().enumerate() {
            hex.push_str(&color_hex(*byte));

            if i % 2 != 0 {
                hex.push(' ');
            }
        }

        let bytes_unread: usize = buffer.len() - bytes_read;
        for _ in 1..bytes_unread {
            hex.push(' ');
        }

        print!("{} ", hex);

        let text = str::from_utf8(&buffer)
            .unwrap()
            .trim_start()
            .trim_end()
            .replace(" ", ".");
        // println!("{}", text);
    }

    Ok(())
}

use colored::Colorize;
use std::fs::File;
use std::io::{self, Read};

/// Turn a byte into its colored,
/// 2-digit hex representation
fn color_hex(byte: u8) -> String {
    // format a byte into a 2-digit hex string
    // https://doc.rust-lang.org/std/fmt/#sign
    let formatted_byte = format!("{:02x}", byte);

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

/// Turns a byte into its colored
/// text representation like in `xxd`
fn color_text(byte: u8) -> String {
    // https://www.marcusfolkesson.se/til/xxd-color-support
    let c = match byte {
        31..127 => char::from(byte),
        _ => '.',
    }
    .to_string();

    match byte {
        31..127 => c.green().to_string(),
        9 | 10 | 13 => c.yellow().to_string(),
        0 => c.white().to_string(),
        255 => c.blue().to_string(),
        _ => c.red().to_string(),
    }
}

/// Read file from argument in std::env::args
/// Handles errors internally
fn get_file_from_args() -> File {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("too many arguments");
        eprintln!("usage: xxd <filename>");
        std::process::exit(1);
    }

    let filename = match args.get(1) {
        Some(filename) => filename.clone(),
        None => {
            eprintln!("file {} not found", args.get(1).unwrap());
            eprintln!("usage: xxd <filename>");
            std::process::exit(1);
        }
    };

    match File::open(&filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("error opening file {}", filename);
            std::process::exit(1);
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = get_file_from_args();

    let mut buffer = [0u8; 16];
    let mut offset: usize = 0;

    // Loop for printing output. Looks like:
    // 00000000: 4861 6c66 2061 206c 6561 6775 652c 2068  Half a league, h
    // ∟ offset  ∟ hex                                    ∟ text
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        // 00000000: ...
        print!("{:08x}: ", offset);
        offset += buffer.len();

        // ... 4861 6c66 2061 206c 6561 6775 652c 2068 ...
        let mut hex = String::new();

        for (i, byte) in buffer.iter().enumerate().take(bytes_read) {
            hex.push_str(&color_hex(*byte));

            if i % 2 != 0 {
                hex.push(' ');
            }
        }

        // Filling whitespace
        let bytes_unread: usize = buffer.len() - bytes_read;
        for i in 0..bytes_unread {
            hex.push(' ');
            hex.push(' ');

            if i % 2 != 0 {
                hex.push(' ');
            }
        }

        print!("{} ", hex);

        // ... Half a league, h
        let mut text = String::new();

        for byte in buffer.iter().take(bytes_read) {
            text.push_str(&color_text(*byte));
        }

        println!("{}", text);
    }

    Ok(())
}

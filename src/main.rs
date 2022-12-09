use std::io::{Read, Write};

use clap::{Arg, ArgAction, Command};

fn main() {
    let app = Command::new("base64")
        .version("1.0")
        .about("Encode/decode between base64 text and binary/text")
        .arg(
            Arg::new("decode_mode")
                .long("decode")
                .short('d')
                .help("Decode base64 text to UTF-8 text or bytes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("text")
                .help("UTF-8 text to encode; or base64 text to decode")
                .index(1),
        )
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .help("Read from file"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Write to file"),
        );

    let matches = app.get_matches();

    let decode_mode = matches.get_flag("decode_mode");

    let text_option = matches.get_one::<String>("text").map(|s| s.as_str());
    let input_option = matches.get_one::<String>("input").map(|s| s.as_str());
    let output_option = matches.get_one::<String>("output").map(|s| s.as_str());

    if decode_mode {
        //
        // DECODE from base64 text
        //
        if let Some(filename) = input_option {
            // file provided
            let buf = read_from_file(filename);
            decode_base64(&buf, output_option);
        } else if let Some(text) = text_option {
            // text provided
            decode_base64(text, output_option);
        } else {
            // read from stdin
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            decode_base64(&buffer, output_option);
        }
    } else {
        //
        // ENCODE to base64 text
        //
        if let Some(filename) = input_option {
            // file provided
            let buf = read_from_file(filename);
            encode_base64(&buf, output_option);
        } else if let Some(text) = text_option {
            // text provided
            encode_base64(text, output_option);
        } else {
            // read stdio
            let mut buf: Vec<u8> = Vec::new();
            std::io::stdin().read_to_end(&mut buf).unwrap();
            encode_base64(buf, output_option);
        }
    }
}

fn encode_base64<T: AsRef<[u8]>>(input: T, output_option: Option<&str>) {
    let b64_str = base64::encode(input);
    if let Some(filename) = output_option {
        // write to file
        write_to_file(filename, b64_str.as_bytes());
    } else {
        // write to stdout
        print!("{}", b64_str);
    }
}

fn decode_base64<T: AsRef<[u8]>>(input: T, output_option: Option<&str>) {
    match base64::decode(input) {
        Ok(bytes) => {
            if let Some(filename) = output_option {
                // write to file
                write_to_file(filename, bytes.as_slice());
            } else {
                // write to stdout
                match std::io::stdout().write_all(&bytes) {
                    Ok(_) => {} // ok
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn read_from_file(filename: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn write_to_file(filename: &str, data: &[u8]) {
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(data).unwrap();
}

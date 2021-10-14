use std::io::{Read, Write};

use clap::{App, Arg};

fn main() {
    let app = App::new("base64")
        .version("1.0")
        .author("Wilson")
        .about("Encode/decode between base64 text and binary/text")
        .arg(
            Arg::new("decode_flag")
                .long("decode")
                .short('d')
                .about("Decode base64 text to UTF-8 text or bytes"),
        )
        .arg(
            Arg::new("text")
                .about("UTF-8 text to encode; or base64 text to decode")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .about("Read from file")
                .takes_value(true),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .about("Write to file")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let decode_flag = matches.occurrences_of("decode_flag");

    let text_option = matches.value_of("text");
    let input_option = matches.value_of("input");
    let output_option = matches.value_of("output");

    if decode_flag > 0 {
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
                std::io::stdout().write_all(&bytes).unwrap();
            }
        }
        Err(e) => {
            println!("{}", e);
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

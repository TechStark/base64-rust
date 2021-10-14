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
                .about("Decode base64 text to binary/text"),
        )
        .arg(
            Arg::new("text")
                .long("text")
                .short('t')
                .about("Encode text as UTF-8 bytes then to base64; or decode base64 text")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let decode_flag = matches.occurrences_of("decode_flag");

    if let Some(text) = matches.value_of("text") {
        if decode_flag > 0 {
            // decode base64 text to binary/text
            match base64::decode(text) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(string) => println!("{}", string),
                    Err(err) => println!("Failed to convert decoded bytes to UTF-8 text: {}", err),
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        } else {
            // encode to base64 text
            let bas64_str = base64::encode(text);
            println!("{}", bas64_str);
        }
    }
}

mod utils;
mod coding;

use std;
use std::io::Write;

use utils::counter;
use coding::arithmetic;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut text: String = String::new();

    match args.len() {
        1 => {
            print!("Enter text: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut text).unwrap();
        },

        2 => {
            text = args[1].clone();
        },

        _ => {
            text = "Hello world".to_string();
        }
    }
    let char_counter = counter::char_counter(text.as_str());

    let code = arithmetic::encode(&char_counter, text.as_str());
    println!("Encoded {}", code);

    let decoded = arithmetic::decode(&char_counter, &code);
    println!("Decoded {}", decoded);

}
mod utils;
mod coding;

use utils::counter;
use coding::arithmetic;

fn main() {
    let str = "Hello world🍑";
    println!("{} {}", str, str.len());
    let char_counter = counter::char_counter(str);
    println!("{:?}", char_counter);
    let str_size: usize = char_counter.values().sum();
    println!("{}", str_size);
    let code = arithmetic::encode(&char_counter, str);
    println!("Encoded {}", code);

    let decoded = arithmetic::decode(&char_counter, &code);
    println!("Decoded {}", decoded);

}
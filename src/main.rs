pub mod read_input;
use crate::read_input::read_args;
fn main() {
    let input = read_args::read_input_file();
    println!("{}", input);
}

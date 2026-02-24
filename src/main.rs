pub mod eval;
pub mod f64plus;
pub mod parse;
pub mod rational;
pub mod utils;
pub mod value;

fn main() {
    let tokens = parse::lex("5647");

    dbg!(tokens);
}

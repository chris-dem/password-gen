#![allow(dead_code)]

use clap::Parser;
use rand::{prelude::*,seq::SliceRandom};
use std::str::Chars;

const UCHARS : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LCHARS : &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS : &str = "0123456789";
const SYMB   : &str = "~`! @#$%^&*()_-+={[}]|\\:;\"'<,>.?/";

///Simple programm to generate passwords
/// By default, lower case latin characters are used
#[derive(Parser,Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {

    /// Allow upper case characters
    #[clap(short,action)]
    upper : bool,

    /// Allow digits
    #[clap(short,action)]
    digits : bool,

    /// Allow symbols
    #[clap(short,action)]
    syms   : bool,

    /// Length of password to generate
    #[clap(short,long,value_parser, default_value_t = 16)]
    length : u8
}


fn passwd_gen(chrs : &[char]) -> String {
    let mut rng = thread_rng();

    let n : u8 = rng.gen_range(8..=20);
    
    chrs.choose_multiple(&mut rng, n as usize).collect()
}




fn main() {
    let args = Args::parse();



    // for _ in 0..5 {
    //     println!("Generated password {}", passwd_gen(&alphabet))
    // }
}



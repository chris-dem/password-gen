#![allow(dead_code)]

use clap::Parser;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use itertools::Itertools;
use rand::{prelude::*/*,seq::SliceRandom*/};
use std::collections::LinkedList;

const UCHARS : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LCHARS : &'static str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS : &'static str = "0123456789";
const SYMB   : &'static str = "~`! @#$%^&*()_-+={[}]|\\:;\"'<,>.?/";

/// Simple programm to generate passwords
/// MAX PASS LEN : 50
/// MAX NUMBER OF PASSWORDS TO GENERATE : 25
/// By default, lowercase latin characters are used
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
    /// If not set, generates password of random length between 8..20
    #[clap(short,long,value_parser)]
    length : Option<u8>,

    /// Number of passwords to generate
    #[clap(short,value_parser, default_value_t = 1)]
    n : u8,

    /// Input Catchphrase
    #[clap(short,action)]
    phrase  : bool,

    /// Custom Character set
    #[clap(short,long, value_parser)]
    chars   : Option<String>,

    // /// Save passwords to a file
    // #[clap(short, action)]
    // save    : bool,
}


fn passwd_gen(chrs : &str, rng : &mut ThreadRng, choose : Option<u8>) -> String {

    let n : u8 = choose.unwrap_or(rng.gen_range(8..=20));
    
    String::from_iter(chrs.chars().choose_multiple(rng, n as usize))
}

fn create_string(args : &Args) -> String {
 
    let mut stream = LCHARS.to_owned();

    if args.upper {
        stream.push_str(UCHARS)
    }

    if args.digits {
        stream.push_str(DIGITS)
    }

    if args.syms {
        stream.push_str(SYMB)
    }

    stream
}

fn remove_dub(st : String) -> String {
    st.chars().sorted().dedup().collect()
}

fn read_from_file(reader : BufReader<File>) -> Result<Vec<LinkedList<String>>,std::io::Error> {
    let mut ret = (0..27).map(|_| LinkedList::new()).collect_vec();
    for line in reader.lines() {
        let st = line?.to_ascii_lowercase();
        let  x = st.chars().nth(0).unwrap();
        ret[x as usize - 'a' as usize].push_back(st);
    }
    Ok(ret)
}

fn generate_catch(words : &VecLis, word : &str, rng : &mut ThreadRng) -> String {
    word.chars().fold(String::new(),|mut acc,c| {
        if c.is_alphabetic() {
            let c = c.to_ascii_lowercase();
            let w = words[c as usize - 'a' as usize].iter().choose(rng).expect("Words for specific letter not found").clone();
            acc.push_str(format!(" {}", w).as_str());
        }else {
            acc.push_str(format!(" {}",c).as_str());
        }
        acc
    })
}

type VecLis = Vec<LinkedList<String>>;

fn main() {
    let args = Args::parse();
    assert!(args.length.unwrap_or(50) <= 50);
    assert!(args.n <= 25);

    let char_arr = match args.chars {
        Some(v) => remove_dub(v),
        None => create_string(&args)
    };
    if char_arr.len() == 0 {
        panic!("Charset cannot be empty")
    }
    
    let file = fs::OpenOptions::new().write(true).read(true).open("assets/wordlist.txt").expect("Wordlist not found");
    let words = read_from_file(BufReader::new(file)).expect("Read from file failed"); 
    let mut rng = thread_rng();

    for _ in 0..args.n {
        let st = passwd_gen(char_arr.as_str(), &mut rng, args.length);
        if !args.phrase {
            println!("{}",st);
        } else {
            println!("{} : {}",st, generate_catch(&words,&st, &mut rng))
        }
    }
}



use crate::consts::*;
use crate::parser::jp_md::*;

mod errors;
mod prelude;
mod utils;
mod parser;
mod consts;
mod convert;
mod anki;

fn main() {
    //parse_jp(STR1);
    run1();
}

fn run1() {
    let input = STR1;
    let list_res = parse_items(input).and_then(|(_, items)| { return Ok(items); });
    match list_res {
        Ok(mut list) => {
            let pair_list: Vec<(&str, &str)> = list.as_mut_slice().into_iter().map(|w: &mut WordExplanation| {
                w.convert();
                (w.word.as_str(), w.explanation.as_str())
            }).collect();
            anki::anki::create_apkg(pair_list).expect("create anki apkg failed.");
        }
        Err(_) => { eprintln!("parse word list error.") }
    }
}

fn parse_jp(s: &str) {
    println!("{:?}", STR1);
    s.lines().map(|line| { println!("{}", line) }).for_each(drop);
}

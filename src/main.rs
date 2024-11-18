use prelude::{Result, W};
use std::fs;
use nom::character::complete::space1;
use nom::error::Error;
use crate::consts::*;
use crate::parser::parser::*;
use crate::parser::jp_md::*;
use crate::convert::convert::convert_one_word_with_ann_and_extra_str;
use crate::prelude::f;
use genanki_rs::{Note, Error as ankiError};

mod errors;
mod prelude;
mod utils;
mod parser;
mod consts;
mod convert;
mod anki;

fn main() {
    //parse_jp(STR1);
    test1();
}

fn test1() {
    let input = STR1;
    let list_res = parse_items(input).and_then(|(_, items)| { return Ok(items); });
    match list_res {
        Ok(list) => {
            for mut w in list {
                let a = w.convert();
                println!("{:?}", w);
            }

            // for w in list {
            //     // 转换单词，转换单词含义，组装 anki 内容 todo
            //     let mut jp_word = w.word.to_string();
            //     if (&w).word.contains("{") {
            //         jp_word = convert_one_word_with_ann_and_extra_str(&w.word).and_then(|(_, s)| { Ok(s) }).unwrap();
            //     }
            //
            //     println!("{}", jp_word);
            // }
        }
        Err(_) => { eprintln!("get word list error.") }
    }
    // let res = parse_items(input);

}

fn parse_jp(s: &str) {
    println!("{:?}", STR1);
    s.lines().map(|line| { println!("{}", line) }).for_each(drop);
}

/// read file list in some dir
/// test function. You can delete it
fn get_files_by_dir(dir: String) -> Result<Vec<String>> {
    let mut list = vec![];
    for entry in fs::read_dir(dir)?.filter_map(|item| item.ok()) {
        let entry: String = W(&entry).try_into()?;
        list.push(entry);
    }
    return Ok(list);
}

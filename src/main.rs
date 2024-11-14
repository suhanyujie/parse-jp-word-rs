use prelude::{Result, W};
use std::fs;
use nom::character::complete::space1;
use crate::consts::*;
use crate::parser::parser::*;
use crate::parser::jp_md::*;

mod errors;
mod prelude;
mod utils;
mod parser;
mod consts;

fn main() {
    //parse_jp(STR1);
    test1();
}

fn test1() {
    let res = "1111a[aa](as)".find(&['\\', 'a'][..]).unwrap_or_default();
    // - クーラー：空调，冷气设备\n- テレビ：电视\n
    let input = "- 貴方：貴方 \n- 貴方：貴方\n"; // [aa](http://as.com)
    let input = STR1;
    // let res = parts_word1(input);
    let res = parse_items(input);

    println!("111 {:#?}", res.unwrap_or_default())
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

use crate::anki::anki::{gen_anki_card_for_kanji, gen_anki_card_for_kanji_pattern_asm};
use crate::parser::jp_md::*;
use clap::{arg, Parser, Subcommand};
use prelude::*;

mod errors;
mod prelude;
mod utils;
mod parser;
mod consts;
mod convert;
mod anki;

/// Japanese word convert into anki card.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "goi", help = "sub command type. eg: `goi`, `kanji` etc.")]
    sub: String,
    #[arg(
        short = 'b',
        long = "book_name",
        default_value = "学ぼうー日本語中級",
        help = "book's name or deck name. eg: `学ぼうー日本語中級`, `学ぼうー日本語中上級` or `testDeck::01` etc."
    )]
    book_name: String,
    #[arg(short, long, default_value = "21", help = "type's chapter number. eg: 21, 22 etc.")]
    num: usize,
    #[arg(
        short, long,
        help = "target file for parse. eg: `./data/kanji.txt`",
        default_value = "01"
    )]
    file: String,
    #[arg(short, long, default_value = "queryAnon", help = "parse pattern. eg: `asm`, `queryAnon` etc.")]
    pattern: String,
}

#[derive(Subcommand)]
enum Commands {
    Type {}
}

fn main() {
    cli_main();
}

fn cli_main() {
    // let matches = Command::new("jwa")
    //     .version("1.0")
    //     .about("Japanese word convert into anki card.");
    let args = Cli::parse();
    let output = match args.sub.as_str() {
        "goi" => {
            goi_handler(&args)
        }
        "kanji" => {
            kanji_handler(&args)
        }
        "other" => {
            other_handler(&args)
        }
        _ => {
            eprintln!("not supported. ");
            Ok("error. ".to_string())
        }
    }.expect("execute failed");
    println!("{}", output);
}

fn other_handler(param: &Cli) -> Result<String> {
    let book_name = param.book_name.as_str();
    let deck_name_prev = f!("{}", book_name);
    let deck_name = deck_name_prev.as_str();
    println!("convert anki card. deck name: {}", deck_name);
    let pattern = param.pattern.as_str();
    let res = gen_anki_card_for_kanji_pattern_asm(param.file.as_str(), deck_name);
    Ok("ok".to_string())
}

fn goi_handler(param: &Cli) -> Result<String> {
    let book_name = param.book_name.as_str();
    let mut deck_name_prev = f!("{}::語彙", book_name);
    deck_name_prev.push_str(param.num.to_string().as_str());
    let deck_name = deck_name_prev.as_str();
    println!("convert anki card. deck name: {}", deck_name);
    let res = gen_anki_card_for_kanji(param.file.as_str(), deck_name);
    Ok("ok".to_string())
}

fn kanji_handler(param: &Cli) -> Result<String> {
    let book_name = param.book_name.as_str();
    let mut deck_name_prev = f!("{}::漢字", book_name);
    deck_name_prev.push_str(param.num.to_string().as_str());
    let deck_name = deck_name_prev.as_str();
    println!("convert anki card. deck name: {}", deck_name);
    let res = gen_anki_card_for_kanji(param.file.as_str(), deck_name);
    Ok("ok".to_string())
}

fn run1(param: &Cli) -> Result<String> {
    // let book_name = param.book_name.as_str();
    // let deck_name = book_name;
    // println!("convert anki card. deck name: {}", deck_name);
    // let pattern = param.pattern.as_str();
    // let res = gen_anki_card_for_kanji_pattern_asm(param.file.as_str(), deck_name);
    // Ok("ok".to_string())

    let mut source_file = param.file.as_str();
    if source_file.len() < 1 {
        source_file = "./data/tmp.txt";
    }
    let cont = std::fs::read_to_string(source_file)?;
    let input = cont.as_ref();
    let list_res = parse_items(input).and_then(|(_, items)| { return Ok(items); });
    match list_res {
        Ok(mut list) => {
            let pair_list: Vec<(&str, &str)> = list.as_mut_slice().into_iter().map(|w: &mut WordExplanation| {
                w.convert();
                (w.word.as_str(), w.explanation.as_str())
            }).collect();
            anki::anki::create_apkg(pair_list, "学ぼうー日本語中級::test-1").expect("create anki apkg failed.");
        }
        Err(_) => { eprintln!("parse word list error.") }
    }

    Ok("ok".to_string())
}


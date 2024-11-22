use crate::anki::anki::gen_anki_card_for_kanji;
use crate::consts::*;
use crate::parser::jp_md::*;
use clap::{arg, Command, Parser, Subcommand};
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
    #[arg(short, long, default_value = "21", help = "type's chapter number. eg: 21, 22 etc.")]
    num: usize,
    #[arg(
        short, long,
        help = "target file for parse. eg: `./data/kanji.txt`"
    )]
    file: String,
}

#[derive(Subcommand)]
enum Commands {
    Type {}
}

fn main() {
    //parse_jp(STR1);
    // run1();
    cli_main();
}

fn cli() {}

fn cli_main() {
    // let matches = Command::new("jwa")
    //     .version("1.0")
    //     .about("Japanese word convert into anki card.");
    let args = Cli::parse();
    let output = match args.sub.as_str() {
        "goi" => {
            goi_handler(&args)
        }
        _ => {
            eprintln!("not supported. ");
            Ok("error. ".to_string())
        }
    }.expect("execute failed");
    println!("{}", output);
}

fn goi_handler(param: &Cli) -> Result<String> {
    let mut deck_name_prev = "学ぼうー日本語中級::語彙".to_string();
    deck_name_prev.push_str(param.num.to_string().as_str());
    let deck_name = deck_name_prev.as_str();
    println!("convert anki card. deck name: {}", deck_name);
    let res = gen_anki_card_for_kanji(param.file.as_str(), deck_name);
    Ok("ok".to_string())
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
            anki::anki::create_apkg(pair_list, "学ぼうー日本語中級::test-1").expect("create anki apkg failed.");
        }
        Err(_) => { eprintln!("parse word list error.") }
    }
}


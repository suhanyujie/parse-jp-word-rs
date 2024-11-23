use lindera::mode::{Mode};
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use lindera::dictionary::{
    load_dictionary_from_kind, DictionaryKind,
};
use crate::prelude::*;
use crate::parser::jp_md::CharJpExt;
use std::borrow::Borrow;

pub fn convert_file(file_path: &str) -> Result<(Vec<String>, Vec<String>)> {
    let mut cont = std::fs::read_to_string(file_path)?;
    let mut meaning_list = vec![];
    if cont.contains("---") {
        let word_cont: Vec<String> = cont.split("---").map(|w| w.trim().to_string()).collect();
        cont = word_cont[0].clone();
        meaning_list = word_cont[1].lines().map(|w| w.trim().to_string()).collect();
    }
    let word_list = convert_words(cont.as_str())?;

    Ok((word_list, meaning_list))
}


pub fn convert_words(input: &str) -> Result<Vec<String>> {
    let res: Vec<String> = input.lines().map(|line| {
        let one = convert_one_word(line).expect("failed to convert one word.");
        return one;
    }).collect();
    Ok(res)
}

fn convert_one_word(line: &str) -> Result<String> {
    if !has_kanji(line) {
        return Ok(line.to_string());
    }
    let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;
    let segmenter = Segmenter::new(Mode::Normal, dictionary, Option::None);
    let tokenizer = Tokenizer::new(segmenter);
    let mut tokens = tokenizer.tokenize(line)?;

    let mut index = 0;
    let mut new_str = String::new();
    for token in tokens.iter_mut() {
        let text = token.text.as_ref().to_string();

        let reading = if let Some(reading) = token.get_detail(7) {
            reading
        } else if text.clone().is_ascii() {
            text.as_ref()
        } else {
            let s = text.clone();
            eprintln!("error word：{}, error char: {}", line, s);
            text.as_ref()
        };


        if has_kanji(text.as_str()) {
            let prefix_space = if index == 0 { "" } else { " " };
            let part = f!("{}{}[{}]", prefix_space, text, str_to_hiragana(reading.as_ref()));
            new_str.push_str(part.as_str());
        } else {
            new_str.push_str(text.as_str());
        }
        index += 1;
    }
    Ok(new_str)
}

fn has_kanji(s: &str) -> bool {
    let mut has_kanji = false;
    let c_arr = s.chars();
    for ch in c_arr {
        if ch.is_kanji() {
            has_kanji = true;
            break;
        }
    }
    has_kanji
}

fn str_to_hiragana(s: &str) -> String {
    let mut hiragana = String::new();
    let c_arr = s.char_indices();
    for (_, ch) in c_arr {
        if (0x30A1..=0x30F6).contains(&(ch as u32)) {
            // 将片假名字符转换为平假名
            let new_c = std::char::from_u32((ch as u32) - 0x60).expect("char convert failed.");
            hiragana.push(new_c);
        }
    }
    hiragana
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_one_word() {
        let res = convert_one_word("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です");
        println!("{:?}", res.unwrap());
    }

    #[test]
    fn test_str_to_hiragana() {
        let res = str_to_hiragana("ー");
        println!("{:?}", res);
    }
    #[test]
    fn test_convert_file() {
        let (res, _) = convert_file("./data/kanji.txt").unwrap();
        println!("{}", res.as_slice().join("\n"));
        assert!(res.len() > 0);
    }
}
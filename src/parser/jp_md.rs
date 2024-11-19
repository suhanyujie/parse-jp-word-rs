use nom::sequence::{preceded, Tuple};
use nom::character::complete::{char, line_ending, not_line_ending, space1};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair};
use crate::convert::convert::{convert_one_word_with_ann_and_extra_str, one_line_to_html};
use crate::parser::parser::*;
use crate::prelude::*;

pub fn parts_word1(input: &str) -> IResult<&str, &str> {
    let mut index = 0;
    let mut it1 = input.chars();
    while let Some(c) = it1.next() {
        if !c.is_jp_char() {
            break;
        }
        index += c.len_utf8();
    }
    Ok((&input[index..], &input[..index]))
}

pub fn jp_string(input: &str) -> IResult<&str, String> {
    let mut index = 0;
    let mut it1 = input.chars();
    while let Some(c) = it1.next() {
        if !c.is_jp_char() {
            break;
        }
        index += c.len_utf8();
    }
    if index == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )));
    }
    let res = (&input[..index]).to_string();
    Ok((&input[index..], res))
}

pub fn parts_meaning(input: &str) -> IResult<&str, (&str, &str)> {
    (not_line_ending, line_ending).parse(input)
}

pub fn parts_item_line(input: &str) -> IResult<&str, (&str, &str, &str, (&str, &str))> {
    (tag("- "), parts_word1, tag("："), parts_meaning).parse(input)
}


pub fn markdown_link(input: &str) -> IResult<&str, (&str, &str)> {
    pair(
        fenced("[", "]"),
        delimited(char('('), take_until_unbalanced('(', ')'), char(')')),
    )(input)
}


// 定义一个结构体来存储解析出的单词和解释
#[derive(Debug)]
pub struct WordExplanation {
    pub is_converted: bool,
    pub word: String,
    pub explanation: String,
    pub sub_items: Vec<String>,
}

impl Default for WordExplanation {
    fn default() -> Self {
        Self {
            is_converted: false,
            word: "".to_string(),
            explanation: "".to_string(),
            sub_items: vec![],
        }
    }
}

impl WordExplanation {
    // 转换单词，及其“意思”为 html
    pub fn convert(&mut self) {
        self.is_converted = true;
        self.word = convert_one_word_with_ann_and_extra_str(self.word.as_str()).and_then(|(_, s)| Ok(s)).unwrap_or_default();
        self.explanation = one_line_to_html(self.explanation.as_str()).unwrap_or_default();
        if self.sub_items.len() > 0 {
            let new_subs: Vec<String> = self.sub_items.iter().map(|item| {
                let mut ori_s = one_line_to_html(&item).unwrap();
                ori_s = ori_s.replace("<p>", "<li>");
                ori_s = ori_s.replace("</p>", "</li>");
                ori_s
            }).collect();
            self.explanation = f!("{}{}", self.explanation, (&new_subs).clone().join(""));
            self.sub_items = new_subs;
        }
    }
}

// 解析每个条目，例如：`- クーラー：空调，冷气设备`
pub fn parse_entry(input: &str) -> IResult<&str, WordExplanation> {
    let (input, _) = opt(line_ending)(input)?;
    let (input, _) = alt((tag("- "), tag("* ")))(input)?; // 匹配列表项的开头 `- `
    let (input, word) = take_until("：")(input)?; // 取出日语单词，直到 `：`
    let (input, _) = char('：')(input)?; // 跳过 `：`
    let (input, explanation) = not_line_ending(input)?; // 获取解释，直到行尾
    let (input, _) = line_ending(input)?; // 读取换行符
    // 尝试解析子条目
    let (input, sub_items) = opt(many1(preceded(space1, parse_sub_entry)))(input)?;

    Ok((
        input,
        WordExplanation {
            word: word.to_string(),
            explanation: explanation.to_string(),
            sub_items: sub_items.unwrap_or_default(),
            ..WordExplanation::default()
        },
    ))
}

// 解析子条目，例如：`  - 自動モード：自动模式`
pub fn parse_sub_entry(input: &str) -> IResult<&str, String> {
    // let (input, _) = space1(input)?; // 匹配缩进
    let (input, _) = parse_item_prefix(input)?;
    sub_item_entry_content(input)
}

fn parse_item_prefix(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("- "), tag("* ")))(input)?;
    Ok((input, ()))
}

fn sub_item_entry_content(input: &str) -> IResult<&str, String> {
    let (input, (content, _)) = (not_line_ending, line_ending).parse(input)?;
    Ok((input, content.to_string()))
}

// 解析整个输入，返回一个 `WordExplanation` 的向量
pub fn parse_items(input: &str) -> IResult<&str, Vec<WordExplanation>> {
    many0(parse_entry)(input) // 使用 many0 来解析多个条目
}


pub trait CharJpExt {
    fn is_jp_char(&self) -> bool;
    fn is_kanji(&self) -> bool;
}

impl CharJpExt for char {
    fn is_jp_char(&self) -> bool {
        matches!(*self,
            '\u{3040}'..='\u{309F}'  // 平假名
            | '\u{30A0}'..='\u{30FF}' // 片假名
            | '\u{3000}'..='\u{303F}' // 日文符号和标点
            | '\u{FF00}'..='\u{FFEF}' // 全角 ASCII 和标点
            | '\u{4E00}'..='\u{9FFF}' // CJK 统一表意文字
        )
    }

    fn is_kanji(&self) -> bool {
        matches!(*self,
            | '\u{3400}'..='\u{4dbf}' // CJK 扩展 A 区
            | '\u{f900}'..='\u{faff}' // CJK 兼容表意文字
            | '\u{4E00}'..='\u{9FFF}'
        )
    }
}



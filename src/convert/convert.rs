use nom::sequence::Tuple;
use nom::bytes::complete::{tag, take_until};
use nom::{IResult, Parser};
use nom::branch::{alt};
use nom::multi::many1;
use crate::parser::jp_md::{jp_string};
use crate::prelude::*;
use markdown::to_html;

// word: {一応|いちおう}
// convert into ruby html string
// ann -> Annotation
pub fn convert_one_word_with_ann(word_str: &str) -> IResult<&str, String> {
    let (input, (_, word_part, _, reading_part, _)) = (tag("{"), take_until("|"), tag("|"), take_until("}"), tag("}")).parse(word_str)?;
    // 「明日」->  `<ruby> 明日 <rp>(</rp><rt>あした</rt><rp>)</rp> </ruby>`
    let ruby_html = f!(
        "<ruby>{}<rp>(</rp><rt>{}</rt><rp>)</rp></ruby>",
        word_part, reading_part
    );
    Ok((input, ruby_html))
}

// word: `何が{一応|いちおう}ですか？`
pub fn convert_one_word_with_ann_and_extra_str(word_str: &str) -> IResult<&str, String> {
    let (input, list) = many1(alt((jp_string, convert_one_word_with_ann))).parse(word_str)?;
    let s = list.iter().fold(String::new(), |acc, d| acc + &d.to_string());
    Ok((input, s))
}

pub fn one_line_to_html(input: &str) -> Result<String> {
    let html_str = to_html(input);
    Ok(html_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_one_word_with_ann() {
        let res = convert_one_word_with_ann("{一応|いちおう}");
        println!("{:?}", res);
    }

    #[test]
    fn test_convert_one_word_with_ann_and_extra_str() {
        // {一応|いちおう}{一応|いちおう}ですか
        if let Ok((i, s)) = convert_one_word_with_ann_and_extra_str("一応{一応|いちおう}いち{一応|いちおう}") {
            println!("{:?}", s);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_one_line_to_html() {
        let input = "[名] 看清，看透。";
        let res = one_line_to_html(input);
        println!("{:?}", res);
    }
}
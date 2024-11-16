use nom::sequence::{preceded, Tuple};
use nom::bytes::complete::{tag, take_until};
use nom::{IResult, Parser};
use nom::branch::{alt, permutation};
use nom::combinator::complete;
use nom::multi::{many0, many1};
use crate::parser::jp_md::{jp_string, parts_word1};
use crate::prelude::*;

// word: {一応|いちおう}
// convert into ruby html string
// ann -> Annotation
pub fn convert_one_word_with_ann(word_str: &str) -> IResult<&str, String> {
    let (input, (_, word_part, _, reading_part, _)) = (tag("{"), take_until("|"), tag("|"), take_until("}"), tag("}")).parse(word_str)?;
    // <ruby> 明日 <rp>(</rp><rt>Ashita</rt><rp>)</rp> </ruby>
    let ruby_html = f!(
        "<ruby>{}<rp>(</rp><rt>{}</rt><rp>)</rp></ruby>",
        word_part, reading_part
    );
    Ok((input, ruby_html))
}

// word: `何が{一応|いちおう}ですか？`
pub fn convert_one_word_with_ann_and_extra_str(word_str: &str) -> IResult<&str, Vec<String>> {
    let (input, list) = many1(alt((jp_string, convert_one_word_with_ann))).parse(word_str)?;
    Ok((input, list))
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
        if let Ok((i, list)) = convert_one_word_with_ann_and_extra_str("一応{一応|いちおう}いち{一応|いちおう}") {
            println!("{:?}", list);
        } else {
            assert!(false);
        }
    }
}
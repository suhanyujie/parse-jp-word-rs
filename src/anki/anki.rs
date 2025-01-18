use genanki_rs::{Field, Model, Template, Deck, Note, Package};
use crate::convert::jp_announce::{convert_file, convert_file_pattern_asm};
use crate::prelude::*;

// generate anki card for kanji and its word
pub fn gen_anki_card_for_kanji(from_file: &str, deck_name: &str) -> Result<()> {
    let mut source_file = from_file;
    if source_file.len() < 1 {
        source_file = "./data/kanji.txt";
    }
    // generate word's announce
    let (words_with_ann, meaning_list) = convert_file(source_file)?;
    // get word's meaning
    let list: Vec<(String, String)> = meaning_list.iter().enumerate().map(|(index, meaning_cont)| {
        let mut infos: Vec<&str> = meaning_cont.as_str().split("：").collect();
        let w = words_with_ann.get(index).expect("get word failed.");
        (w.to_string(), infos.get(1).unwrap().to_string())
    }).collect();
    // gen anki card
    let list_ref = list.iter().map(|(w, m)| (w.as_ref(), m.as_ref())).collect::<Vec<(&str, &str)>>();
    create_apkg(list_ref, deck_name)
}

// parse line like this: word1: `- meaning01`
pub fn gen_anki_card_for_kanji_pattern_asm(from_file: &str, deck_name: &str) -> Result<()> {
    let mut source_file = from_file;
    if source_file.len() < 1 {
        source_file = "./data/kanji.txt";
    }
    // generate word's announce
    let (words_with_ann, meaning_list) = convert_file_pattern_asm(source_file)?;
    // get word's meaning
    let list: Vec<(String, String)> = meaning_list.iter().enumerate().map(|(index, meaning_cont)| {
        let w = words_with_ann.get(index).expect("get word failed.");
        (w.to_string(), meaning_cont.to_string())
    }).collect();
    // gen anki card
    let list_ref = list.iter().map(|(w, m)| (w.as_ref(), m.as_ref())).collect::<Vec<(&str, &str)>>();
    create_apkg(list_ref, deck_name)
}

pub fn create_apkg(word_list: Vec<(&str, &str)>, deck_name: &str) -> Result<()> {
    // 此 id 可以随便写，唯一即可。重要的是后续的牌组名称。
    let mut my_deck = Deck::new(2059400110,
                                deck_name,
                                "jp word learning.");

    let my_model = get_default_model()?;

    word_list.iter().for_each(|(word, meaning)| {
        let note = Note::new(my_model.clone(), vec![
            word,
            meaning,
        ]).expect("create note error.");
        my_deck.add_note(note);
    });

    // do package
    let mut pkg_obj = Package::new(vec![my_deck], vec![])?;
    pkg_obj.write_to_file("./data/output2014.apkg")?;
    Ok(())
}

// Model for japanese word
pub fn get_default_model() -> Result<Model> {
    let mut my_model = Model::new(1728714695620, "Basic (and reversed card) -jp", vec![
        Field::new("Front"),
        Field::new("Back"),
    ], vec![
        Template::new("Card 1").qfmt("{{furigana:Front}}").afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
        Template::new("Card 2").qfmt("{{Back}}").afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{furigana:Front}}"),
    ]);
    my_model = my_model.css(" @font-face {font-family: IPAexGothic;src: url('_ipaexm.ttf');}}.card {font-family: 'IPAexGothic', 'Source Han Serif JP', arial;font-size: 22px;text-align: center;color: black;background-color: white;}.jp-word {font-family: 'IPAexGothic', 'BIZ UDMincho', 'Source Han Serif JP', 'Source Han Serif CN', serif, arial;text-align:center }.meaning{text-align:center }");
    Ok(my_model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_apkg() {
        let res = create_apkg(vec![("w1", "m1"), ("w2", "m2")], "学ぼうー日本語中級::test-1");
    }

    #[test]
    fn test_gen_anki_card_for_kanji() {
        let deck_name = "学ぼうー日本語中級::漢字10";
        let deck_name40 = "学ぼうー日本語中級::語彙21";
        let res = gen_anki_card_for_kanji("./data/kanji.txt", deck_name40);
        assert!(res.is_ok());
    }

    #[test]
    fn test_gen_anki_card_for_kanji_pattern_asm() {
        let deck_name40 = "test1::test";
        let res = gen_anki_card_for_kanji_pattern_asm("./data/tmp.txt", deck_name40);
        assert!(res.is_ok());
    }
}
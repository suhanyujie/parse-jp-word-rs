use genanki_rs::{Field, Model, Template, Error, Deck, Note, Package};
use crate::prelude::*;

pub fn create_apkg() -> Result<()> {
    // 此 id 可以随便写，唯一即可。重要的是后续的牌组名称。
    let mut my_deck = Deck::new(2059400110,
                                "学ぼうー日本語中級::漢字10",
                                "jp word learning.");

    let my_model = get_default_model()?;
    let my_note = Note::new(my_model, vec![
        "example-word",
        "example-meaning",
    ])?;
    // add many note...
    my_deck.add_note(my_note);

    // do package
    let mut pkg_obj = Package::new(vec![my_deck], vec![])?;
    pkg_obj.write_to_file("output2014.apkg")?;
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
# parse jp word rust

Rust で 日本の単語の解析ツールです。

## eg
- `parseJp2anki.exe -f="./data/tmp.txt" -b="学ぼうー日本語中級" -s=kanji -n=12`
- `parseJp2anki.exe -f="./data/tmp.txt" -b="学ぼうー日本語中上級" -s=goi -n=2`

## 参考

* https://imfeld.dev/writing/parsing_with_nom
    * https://github.com/dimfeld/export-logseq-notes
* https://github.com/rust-bakery/nom/blob/main/doc/making_a_new_parser_from_scratch.md
* https://github.com/suhanyujie/quick-start-rs
* nom sql https://github.com/ms705/nom-sql/blob/master/src/parser.rs
* - 更好的日文分词器实现 https://qiita.com/e10persona/items/fddc795e70a05f3bc907

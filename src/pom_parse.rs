use pom::char_class::space;
use pom::parser::{one_of, sym, Parser};

fn pom_parse() {}

pub mod parse {
    use pom::parser::{list, one_of, sym};
    use pom::Parser;

    enum CrateType {
        Letter(u8),
        None,
    }

    pub fn space() -> Parser<u8, ()> {
        one_of(b" \t\r\n").repeat(1).name("space").discard()
    }

    pub fn space_between_columns() -> Parser<u8, ()> {
        space().repeat(1).name("column_space").discard()
    }
    pub fn space_for_blank_crate() -> Parser<u8, ()> {
        sym(b':').repeat(3).name("column_space").discard()
    }

    pub fn no_crate_column<'a>() ->pom::parser::Parser<'a, u8, u8> {
        sym(b' ').repeat(3).name("no_crate") *  sym(b' ')
    }

    pub fn letter_crate<'a>() -> Parser< u8, u8> {
         sym(b'[')  * one_of(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ") - sym(b']')
    }

    pub fn row<'a>() -> Parser< u8, Vec<CrateType>>{
        // crate_spot() - space_between_columns() - sym(b'\n')
        let array = list(crate_spot(),sym(b' '));
        array
    }
    pub fn crate_spot() -> Parser<u8, CrateType> {
        letter_crate().convert(|letter| CrateType::Letter(letter)) | no_crate_column().convert(|_| CrateType::None)
    }

    pub fn row_end() -> Parser<u8, u8> {
        sym(b'\n')
    }
}
// fn box_parser<'a>(input: &'a String) -> Result<u8, pom::Error> {
fn box_parser(input: &'static str) -> Result<u8, pom::Error> {
    let mut parser = parse::letter_crate();
    parser.parse(input.as_bytes())
}

#[cfg(test)]
pub mod tests {
    use std::{assert_eq, println};
    use crate::pom_parse::{box_parser};
    use pom::parser::{list, none_of, seq, sym};
    use pom::set::Set;
    use crate::pom_parse::parse::crate_spot;
    use super::parse;

    pub fn test_input() -> String {
        r###"
    [V] [G]             [H]
    [Z] [H] [Z]         [T] [S]
    [P] [D] [F]         [B] [V] [Q]
    [B] [M] [V] [N]     [F] [D] [N]
    [Q] [Q] [D] [F]     [Z] [Z] [P] [M]
    [M] [Z] [R] [D] [Q] [V] [T] [F] [R]
    [D] [L] [H] [G] [F] [Q] [M] [G] [W]
    [N] [C] [Q] [H] [N] [D] [Q] [M] [B]
    1   2   3   4   5   6   7   8   9 "###
            .into()
    }
    #[test]
    fn test_pow() {
        let input = b"abcde";
        let parser = sym(b'a') * none_of(b"AB") - sym(b'c') + seq(b"de");
        let output = parser.parse(input);
        assert_eq!(output, Ok((b'b', &b"de"[..])));
        println!("!{:?}!", output);
    }

    #[test]
    fn parset_test() {
        let str = b"asdfasd  asdf ";
        let z = parse::space().parse(str);

        println!("!{:?}!", z);
    }
    #[test]
    fn test_whole_paser() {
        let str = "a[b]asdfa";
        let z = box_parser( str);
        let result = z;
    }

    #[test]
    fn test_no_crate_column() {
        let str = b"[B]     [C] [D]";
        let parser = parse::letter_crate() + parse::no_crate_column() + parse::letter_crate();
        let output = parser.parse(str);
        dbg!("!{:?}!", &output);
        assert!(output.is_ok());
    }
    #[test]
    fn test_crate_spot() {
        let str = b"     [B] ";
        let parser = parse::crate_spot();
        
    }
    #[test]
    fn parse_array() {
        let str = b"[A] [C]     [D]";
        let parser = parse::letter_crate() + parse::no_crate_column() + parse::letter_crate();
        let g = list(crate_spot(), sym(b' '));
        let a = g.parse(str);
        panic!("{:?}",a);
        // assert_eq!(g.parse(str), Ok((vec![b'B', b'C', b'D'], &b""[..])));

    }
}

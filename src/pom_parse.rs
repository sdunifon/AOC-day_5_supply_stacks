use pom::char_class::space;
use pom::parser::{one_of, Parser};

fn pom_parse() {}

mod parse {
    use pom::Parser;
    use pom::parser::one_of;

    fn space<'a>() -> Parser<'a, u8, ()> {
        one_of(b" \t\r\n").repeat(0..).name("space").discard()
    }


    fn crate () -> Parser<u8,() >{
        sym('[').one_of(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ").name("crate").sym(']')
    }
}

#[cfg(test)]
pub mod tests{
    use pom::parser::{none_of, seq, sym};

   pub fn test_input() ->String {
        r###"
    [V] [G]             [H]
    [Z] [H] [Z]         [T] [S]
    [P] [D] [F]         [B] [V] [Q]
    [B] [M] [V] [N]     [F] [D] [N]
    [Q] [Q] [D] [F]     [Z] [Z] [P] [M]
    [M] [Z] [R] [D] [Q] [V] [T] [F] [R]
    [D] [L] [H] [G] [F] [Q] [M] [G] [W]
    [N] [C] [Q] [H] [N] [D] [Q] [M] [B]
    1   2   3   4   5   6   7   8   9 "###.into()

    }
    #[test]
    fn test_pow() {
        let input = b"abcde";
        let parser = sym(b'a') * none_of(b"AB") - sym(b'c') + seq(b"de");
        let output = parser.parse(input);
        assert_eq!(output, Ok( (b'b', &b"de"[..]) ) );
        println!("!{:?}!", output);
    }
}
#[test]
fn parset_test() {
    let str = b"asdfasd  asdf ";
    let z = space().parse(str);

    println!("!{:?}!", z);


}

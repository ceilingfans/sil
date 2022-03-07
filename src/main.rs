mod lexer;
mod util;

fn main() {
    let source = util::read_file("examples/helloWord.sil").unwrap();
    let tokens = lexer::lex(source);

    if let Some(t) = tokens {
        println!("{:#?}", t);
    } else {
        eprintln!("syntax error");
    }
}
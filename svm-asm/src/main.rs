use lexer::Lexer;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod lexer;

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // if args.len() != 3 {
    //     eprintln!("usage: svm-asm [infile] [outfile]");
    //     return;
    // }

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let source =
        std::fs::read_to_string("/home/taylor/dev/rust/svm/examples/isort/isort.asm").unwrap();
    let lexer = Lexer::new(&source);
    let out = lexer.tokenize();
    dbg!(&out);
    // let mut lexer = Lexer::new();
    // lexer.lex_token(&source);
}

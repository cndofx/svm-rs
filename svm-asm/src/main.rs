use std::io::Write;

use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod codegen;
mod lexer;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: svm-asm [infile] [outfile]");
        return;
    }

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let infile = &args[1];
    let outfile = &args[2];

    let source = std::fs::read_to_string(infile).unwrap();
    let tokens = lexer::tokenize(&source);
    dbg!(&tokens);

    let code = codegen::generate(&tokens);
    let outfile = std::fs::File::create(outfile).unwrap();
    write_code(&code, outfile).unwrap();
}

fn write_code<W: Write>(code: &[i32], mut w: W) -> Result<(), std::io::Error> {
    for inst in code {
        let bytes = inst.to_le_bytes();
        w.write_all(&bytes)?;
    }
    Ok(())
}

use log::error;
use vm::VM;

use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: svm [filename]");
        return;
    }

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let filename = &args[1];
    run(filename);

    // run("examples/hello/hello");
}

fn run(filename: &str) {
    let mut vm = VM::new();
    if let Err(e) = vm.load(filename) {
        error!("unable to load program: {e}");
        return;
    }
    vm.run();
}

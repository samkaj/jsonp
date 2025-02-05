use std::env;

use jsonp::parse::Parser;
use jsonp::tokenize::Tokenizer;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <json-file>", args[0]);
        return Err(());
    }

    let source = match std::fs::read_to_string(args[1].as_str()) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("IO error: {}", err);
            return Err(());
        }
    };

    let mut tokenizer = Tokenizer::default();
    let tokens = match tokenizer.tokenize(&source) {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("Tokenizer error: {}", err);
            return Err(());
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(json) => {
            dbg!(json);
        }
        Err(err) => {
            eprintln!("{}", err.0);
        }
    }

    Ok(())
}

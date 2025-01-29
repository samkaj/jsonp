use jsonp::lex::Tokenizer;

fn main() -> Result<(), String> {
    let source = match std::fs::read_to_string("src/sample.json") {
        Ok(source) => source,
        Err(err) => return Err(err.to_string()),
    };

    let mut tokenizer = Tokenizer::new();
    if let Ok(tokens) = tokenizer.tokenize(&source) {
        for tok in tokens {
            println!("{:?}", tok);
        }
    }

    Ok(())
}

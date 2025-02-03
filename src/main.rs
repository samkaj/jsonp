use jsonp::parse::Parser;
use jsonp::tokenize::Tokenizer;

fn main() -> Result<(), String> {
    let source = match std::fs::read_to_string("src/sample.json") {
        Ok(source) => source,
        Err(err) => return Err(err.to_string()),
    };

    let mut tokenizer = Tokenizer::default();
    let tokens = tokenizer.tokenize(&source)?;

    let mut parser = Parser::new(tokens);
    let json = parser.parse()?;
    dbg!(json);
    println!("✅");

    Ok(())
}

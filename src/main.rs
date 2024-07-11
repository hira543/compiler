use compiler::backend::codegen::CodeGenerator;
use compiler::parser::lexer::tokenizer;
use compiler::parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let source_code = std::fs::read_to_string(file_name).expect("Failed to read the source file.");
    println!("compiling source code: \n{}", source_code);
    let (_, tokens) = tokenizer(&source_code).expect("Failed to tokenize the source code.");

    let mut parser = Parser { tokens, current: 0 };
    let ast = match parser.parse_tokens() {
        Ok(ast) => ast,
        Err(e) => {
            println!("Failed to parse tokens: {}", e);
            return;
        }
    };

    let mut code_generator = CodeGenerator::new();
    match code_generator.generate_to_file(&ast, "output.asm") {
        Ok(()) => println!("Assembly code was successfully written to 'output.asm'"),
        Err(e) => println!("Error generating assembly code: {}", e),
    }
}

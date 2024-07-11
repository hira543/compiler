//#![allow(dead_code)]
use crate::parser::token::Token;
use nom::{
    //number::complete::{
    //double,
    //},
    branch::{
        alt,
        //permutation,
    },
    bytes::complete::{
        tag,
        //is_not,
        //escaped_transform,
        //take_while_m_n,
        take_while,
        take_while1,
    },
    //error::{ParseError},
    character::complete::{
        //none_of,
        char,
        //line_ending,
        digit1,
        //space0,
        //space1,
        multispace0,
        multispace1,
    },

    combinator::{
        //opt,
        map,
        //value,
        //all_consuming,
        recognize,
    },
    multi::{
        many0,
        //many1,
        //separated_list0,
    },
    sequence::{
        delimited,
        //preceded,
        //tuple,
        pair,
    },
    //error::VerboseError,
    IResult,
};

pub fn display_tokens(tokens: &[Token]) {
    println!("Generated tokens(lexer output):");
    for (index, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", index, token);
    }
}

// 整数リテラルを解析
fn integer(input: &str) -> IResult<&str, Token> {
    let (input, int_str) = digit1(input)?;
    match int_str.parse::<i32>() {
        Ok(i) => Ok((input, Token::I32(i))),
        Err(_) => {
            let i = int_str.parse::<i64>().unwrap();
            Ok((input, Token::I64(i)))
        }
    }
}

// 文字列リテラルを解析
fn string_literal(input: &str) -> IResult<&str, Token> {
    let string_parser = delimited(ws(char('"')), take_while(|c: char| c != '"'), ws(char('"')));
    map(string_parser, |s: &str| Token::String(s.to_string()))(input)
}

// 空白をスキップする関数
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// '+' トークン
fn plus(input: &str) -> IResult<&str, Token> {
    map(ws(char('+')), |_| Token::Plus)(input)
}

// '-' トークン
fn minus(input: &str) -> IResult<&str, Token> {
    map(ws(char('-')), |_| Token::Minus)(input)
}

// '*'
fn star(input: &str) -> IResult<&str, Token> {
    map(ws(char('*')), |_| Token::Star)(input)
}

// '/'
fn slash(input: &str) -> IResult<&str, Token> {
    map(ws(char('/')), |_| Token::Slash)(input)
}

// '%'
fn modulo(input: &str) -> IResult<&str, Token> {
    map(ws(char('%')), |_| Token::Modulo)(input)
}

// '<'
fn less_than(input: &str) -> IResult<&str, Token> {
    map(ws(char('<')), |_| Token::LessThan)(input)
}

// '>'
fn greater_than(input: &str) -> IResult<&str, Token> {
    map(ws(char('>')), |_| Token::GreaterThan)(input)
}

// '='
fn assignment(input: &str) -> IResult<&str, Token> {
    //println!("Trying assignment with input: {}", input);
    let result = map(ws(char('=')), |_| Token::Assignment)(input);
    // match &result {
    //     Ok((remaining, _)) => println!("Assignment parsed successfully, remaining input: {}", remaining),
    //     Err(_) => println!("Failed to parse assignment"),
    // }
    result
}

// '=='
fn double_equal(input: &str) -> IResult<&str, Token> {
    map(ws(tag("==")), |_| Token::DoubleEqual)(input)
}

// '('
fn l_paren(input: &str) -> IResult<&str, Token> {
    map(ws(char('(')), |_| Token::LParen)(input)
}

// ')'
fn r_paren(input: &str) -> IResult<&str, Token> {
    map(ws(char(')')), |_| Token::RParen)(input)
}

// '{'
fn l_brace(input: &str) -> IResult<&str, Token> {
    map(ws(char('{')), |_| Token::LBrace)(input)
}

// '}'
fn r_brace(input: &str) -> IResult<&str, Token> {
    map(ws(char('}')), |_| Token::RBrace)(input)
}

// ';'
fn semicolon(input: &str) -> IResult<&str, Token> {
    map(ws(char(';')), |_| Token::Semicolon)(input)
}

// ','
fn comma(input: &str) -> IResult<&str, Token> {
    map(ws(char(',')), |_| Token::Comma)(input)
}

// キーワードの解析関数
fn keyword(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("function"), |_| Token::Function),
        map(tag("if"), |_| Token::If),
        map(tag("else"), |_| Token::Else),
        map(tag("while"), |_| Token::While),
        map(tag("return"), |_| Token::Return),
    ))(input)
    .and_then(|(next_input, token)| {
        multispace1(next_input).map(|(final_input, _)| (final_input, token))
    })
}

// 識別子を解析
fn identifier(input: &str) -> IResult<&str, Token> {
    //println!("Trying identifier with input: {}", input);
    let start_parser = take_while1(|c: char| c.is_alphabetic() || c == '_');
    let rest_parser = take_while(|c: char| c.is_alphanumeric() || c == '_');
    let mut combined_parser = recognize(pair(start_parser, rest_parser));

    let result = combined_parser(input);
    // match &result {
    //     Ok((remaining, ident)) => {
    //         println!("Identifier parsed successfully: {}, remaining input: {}", ident, remaining);
    //     },
    //     Err(_) => println!("Failed to parse identifier"),
    // }
    result.map(|(remaining, ident)| (remaining, Token::Ident(ident.to_string())))
}

fn type_declaration(input: &str) -> IResult<&str, Token> {
    let (input, ident_token) = identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, var_type) = ws(alt((tag("i32"), tag("i64"), tag("string"))))(input)?;

    if let Token::Ident(var_name) = ident_token {
        Ok((
            input,
            Token::TypeDeclaration(var_name, var_type.to_string()),
        ))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn print_function(input: &str) -> IResult<&str, Token> {
    map(ws(tag("print")), |_| Token::Print)(input)
}

pub fn tokenizer(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = multispace0(input)?;

    let (remaining_input, mut tokens) = many0(alt((
        type_declaration,
        keyword,
        print_function,
        map(identifier, |ident: Token| match &ident {
            Token::Ident(name) if name == "function" => Token::Function,
            Token::Ident(name) if name == "while" => Token::While,
            Token::Ident(name) if name == "if" => Token::If,
            Token::Ident(name) if name == "else" => Token::Else,
            Token::Ident(name) if name == "return" => Token::Return,
            _ => ident,
        }),
        integer,
        string_literal,
        plus,
        minus,
        star,
        slash,
        modulo,
        less_than,
        greater_than,
        double_equal,
        assignment,
        l_paren,
        r_paren,
        l_brace,
        r_brace,
        semicolon,
        comma,
    )))(input)?;

    //println!("Remaining input: {:?}", remaining_input); // 残りの入力を表示
    //println!("Tokens: {:?}", tokens); // 解析したトークンを表示
    //display_tokens(&tokens);
    // 入力が完全に消費された場合EOFトークンを追加
    if remaining_input.is_empty() {
        tokens.push(Token::EOF);
    }

    Ok((remaining_input, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(integer("123 "), Ok((" ", Token::I32(123))));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier("testVar "),
            Ok((" ", Token::Ident("testVar".to_string())))
        );
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            string_literal("\"hello world\" "),
            Ok(("", Token::String("hello world".to_string())))
        );
    }

    #[test]
    fn test_operators() {
        assert_eq!(plus("+"), Ok(("", Token::Plus)));
        assert_eq!(minus("-"), Ok(("", Token::Minus)));
        assert_eq!(star("*"), Ok(("", Token::Star)));
        assert_eq!(slash("/"), Ok(("", Token::Slash)));
        assert_eq!(modulo("%"), Ok(("", Token::Modulo)));
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(less_than("<"), Ok(("", Token::LessThan)));
        assert_eq!(greater_than(">"), Ok(("", Token::GreaterThan)));
        assert_eq!(double_equal("=="), Ok(("", Token::DoubleEqual)));
    }

    #[test]
    fn test_parentheses_and_braces() {
        assert_eq!(l_paren("("), Ok(("", Token::LParen)));
        assert_eq!(r_paren(")"), Ok(("", Token::RParen)));
        assert_eq!(l_brace("{"), Ok(("", Token::LBrace)));
        assert_eq!(r_brace("}"), Ok(("", Token::RBrace)));
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(semicolon(";"), Ok(("", Token::Semicolon)));
    }

    // #[test]
    // fn test_keywords() {
    //     assert_eq!(keyword("function"), Ok(("", Token::Function)));
    //     assert_eq!(keyword("if"), Ok(("", Token::If)));
    //     assert_eq!(keyword("else"), Ok(("", Token::Else)));
    //     assert_eq!(keyword("while"), Ok(("", Token::While)));
    //     assert_eq!(keyword("return"), Ok(("", Token::Return)));
    // }

    #[test]
    fn test_valid_assignment() {
        let input = "hello = 10;";
        if let Ok((_, tokens)) = tokenizer(input) {
            assert_eq!(
                tokens,
                vec![
                    Token::Ident("hello".to_string()),
                    Token::Assignment,
                    Token::I32(10),
                    Token::Semicolon,
                    Token::EOF,
                ]
            );
        } else {
            panic!("Tokenizer failed to parse the input.");
        }
    }

    #[test]
    fn test_function_def() {
        let input = "
            function add(a:i32, b:i32) 
            { 
                return a + b; 
            };
        ";
        assert!(tokenizer(input).is_ok());
    }

    #[test]
    fn test_print_with_expression() {
        let input = "print(10);";
        if let Ok((_, tokens)) = tokenizer(input) {
            assert_eq!(
                tokens,
                vec![
                    Token::Print,
                    Token::LParen,
                    Token::I32(10),
                    Token::RParen,
                    Token::Semicolon,
                    Token::EOF,
                ]
            );
        } else {
            panic!("Tokenizer failed to parse the input.");
        }
    }

    #[test]
    fn test_integer_i32() {
        assert_eq!(integer("123 "), Ok((" ", Token::I32(123))));
    }

    #[test]
    fn test_integer_i64() {
        assert_eq!(integer("2147483648 "), Ok((" ", Token::I64(2147483648))));
    }

    #[test]
    fn test_type_declaration() {
        let input = "x:i32 = 10;";
        assert_eq!(
            tokenizer(input),
            Ok((
                "",
                vec![
                    Token::TypeDeclaration("x".to_string(), "i32".to_string()),
                    Token::Assignment,
                    Token::I32(10),
                    Token::Semicolon,
                    Token::EOF,
                ]
            ))
        );
    }
}

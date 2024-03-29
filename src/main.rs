use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let tokens = tokenize("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    println!("{:?}", tokens);

    let generated_code = generate(&tokens);
    println!("{}", generated_code);

    let mut outfile = File::create("output.c").expect("failed to open outfile");
    outfile.write(generated_code.as_bytes()).expect("failed to write outfile");

    let cmd_output = Command::new("gcc").arg("-o").arg("bf").arg("output.c").output().expect("fuck");
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,
    Sub,
    Right,
    Left,
    Read,
    Write,
    BeginLoop,
    EndLoop,
}

use self::Token::*;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {}
        }
    }

    tokens
}

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.c"));

    for &token in tokens {
        match token {
            Add => {
                output.push_str("++*ptr;\n")
            }
            Sub => {
                output.push_str("--*ptr;\n")
            }
            Right => {
                output.push_str("++ptr;\n")
            }
            Left => {
                output.push_str("--ptr;\n")
            }
            Read => {
                output.push_str("*ptr=getchar();\n")
            }
            Write => {
                output.push_str("putchar(*ptr);\n")
            }
            BeginLoop => {
                output.push_str("while (*ptr) {\n");
            }
            EndLoop => {
                output.push_str("}\n");
            }
        }
    }

    output.push_str("}\n");

    output
}

mod model;
use model::reader::Reader;
use model::tokenizer::Tokenizer;
use model::parser::CompilationEngine;

use std::{env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.jack>");
        return;
    }

    let input_path = &args[1];
    let output_name = input_path.replace(".jack", ".vm");

    let mut reader = Reader::new();
    reader.input(input_path).expect("Could not open input file");

    let mut tokenized = Vec::new();

    // PASS 1: Build Symbol Table for Labels
    while let Some(line) = reader.read_line() {
        let tokens = Tokenizer::get_token(&line);
        let mut set_tokens: Vec<String> = tokens.lines().map(|line| line.to_string()).collect();
        tokenized.append(&mut set_tokens);
    }

    let mut comp = CompilationEngine::new(tokenized);
    comp.compile_class();
    let res: Vec<String> = comp.get_result();

    for line in res {
        reader.push_output(line.clone());
    }

    reader.output(&output_name);
    println!("Successfully compiled {} -> {}", input_path, output_name);
}
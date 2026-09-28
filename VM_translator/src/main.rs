mod model;
use model::reader::Reader;
use model::translator::Translator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.vm>");
        return;
    }

    let input_path = &args[1];
    let output_name = input_path.replace(".vm", ".asm");

    let mut reader = Reader::new();
    let mut translator = Translator::new(&input_path.replace(".vm", ""));

    reader.input(input_path).expect("Could not open input file");

    // PASS 1: Build Symbol Table for Labels
    while let Some(line) = reader.read_line() {
        let asm_line = translator.translate(&line);
        reader.push_output(asm_line);
    }

    // Write final .hack binary file
    reader.output(&output_name).expect("Failed to write output file");

    println!("Successfully compiled {} -> {}", input_path, output_name);
}
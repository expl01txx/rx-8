use clap::Parser;
use color_print::cprintln;
use serde::Serialize;
pub mod parser;
pub mod codegen;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value_t = false)]
    save_instructions: bool,
}

fn main() {
    cprintln!("<green>[rx]: Reading file...<green>");
    let args = Args::parse();

    let src = std::fs::read_to_string(args.filename.as_str());

    if src.is_err() {
        cprintln!("<red>[rx]: Failed to read file: {}<red>", args.filename);
        panic!();
    }

    let src = src.unwrap();

    cprintln!("<green>[rx]: Parsing...<green>");
    let mut parser = parser::Parser::new();
    let instructions = parser.parse(&src, &args.filename).unwrap();

    if args.save_instructions {
        cprintln!("<green>[rx]: Saving instructions...<green>");
        let _ = std::fs::write("instructions.json", serde_json::to_string(instructions).unwrap());
    }

    cprintln!("<green>[rx]: Generating code...<green>");
    let mut codegen = codegen::Codegen::new(instructions.clone());
    let result = codegen.generate();

    cprintln!("<green>[rx]: Saving code...<green>");
    _ = std::fs::write(args.output, result);

}

pub mod a_command_parser;
pub mod blank_line_remover;
pub mod c_command_parser;
pub mod command_parser;
pub mod comment_remover;
pub mod file_reader;
pub mod hack_file_writer;
pub mod line_parser;
pub mod symbol_parser;
pub mod whitespace_remover;

use crate::file_reader::FileReader;
use crate::line_parser::parse_new_lines;
use crate::comment_remover::remove_comments;
use crate::whitespace_remover::remove_whitespaces;
use crate::blank_line_remover::remove_blank_lines;
use crate::symbol_parser::handle_symbols;
use crate::command_parser::parse_simple;
use crate::command_parser::turn_into_binary;
use crate::hack_file_writer::write_to;

fn main() {
    // read file
    let file_name = "hack.asm";
    println!("Reading file {}", file_name);
    let contents = FileReader::new(file_name).read();

    // line parser
    println!("parsing content {:#?}", contents);
    let commands_lines = parse_new_lines(contents);

    // comment remove
    println!("removing comments");
    let commands_no_comments = remove_comments(commands_lines);

    // white space remove
    println!("remove white spaces");
    let commands_no_whitespace = remove_whitespaces(commands_no_comments);

    // blank line remover
    println!("remove blank lines");
    let clean_commands = remove_blank_lines(commands_no_whitespace);

    // parse symbols
    println!("handling symbols");
    let symbols_done_commands = handle_symbols(clean_commands);
    println!("symbols done {:#?}", symbols_done_commands);

    // parse strings to commands
    println!("strings to commands");
    let hack_commands = parse_simple(symbols_done_commands);
    println!("hack commands {:#?}", hack_commands);

    // turn commands to binary output strings
    println!("to binary");
    let binary_commands = turn_into_binary(hack_commands);
    println!("binary commands {:#?}", binary_commands);

    // write binary strings to out file
    let out_file = "out.hack";
    println!("writing out to {}", out_file);
    write_to(out_file, binary_commands);
}

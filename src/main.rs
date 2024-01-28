use lexer::Lexer;
use parser::Parser;

mod token;
mod util;
mod lexer;
mod parser;
mod ast;

fn main() {
	let lexer = Lexer::init();
	let mut parser = Parser::init(lexer);
	let result = parser.scan_for_statements();

	for i in 0..result.len() {
		println!("{}", result[i])
	};
}
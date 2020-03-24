use std::env;
use std::io;
use std::io::prelude::*;

pub mod ast;
//pub mod compile;
pub mod interpret;
pub mod parse;
pub mod tokenize;

use crate::ast::{Item, Module, Procedure};
use crate::tokenize::tokenize;
use crate::interpret::Scope;

fn main() {
	match env::args().skip(1).next() {
		// start REPL
		None => {
			print!(">>> ");
			unimplemented!();
		}
		
		Some(ref arg) if arg == "-" => {
			let mut source = String::new();
			io::stdin().read_to_string(&mut source).expect("File error");
			
			let tokens = tokenize(&source).expect("Could not tokenize");
			
			println!("{:#?}", tokens);
		}
		
		// interpret file
		Some(file) => {
			use std::fs::read_to_string as open;
			
			let source = open(file).expect("Could not read file");
			let mut tokens = tokenize(&source)
				.expect("Lexer error")
				.into_iter()
				.peekable();
			
			match parse::parse_items(&mut tokens) {
				Ok(mut ast) => {
					println!("AST successfully created.");
					
					// look for main function
					let main_pos = ast.iter()
						.position(|item| matches!(
							item,
							Item::Proc(p) if p.name == "main"
						));
					
					// run main procedure, if any
					if let Some(pos) = main_pos {
						let main = ast.remove(pos);
						
						// create root module
						let root_mod = Module {
							name: String::from("root"),
							items: ast,
						};
						
						if let Item::Proc(pr) = main {
							println!("Running...");
							pr.call(Vec::new(), &root_mod);
						} else {
							unreachable!();
						}
					} else {
						eprintln!("No main procedure found.");
					}
				}
				Err(e) => {
					let remaining_tokens = tokens.clone()
						.collect::<Box<[_]>>();
					eprintln!("Expected {}.", e);
					eprintln!("Tokens: {:#?}", remaining_tokens);
				}
			}
		}
	} 
}

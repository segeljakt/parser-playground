#![allow(bad_style)]
#![feature(box_syntax)]
#![feature(try_from)]
#![allow(unused)]


#[macro_use]
extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;
#[macro_use]
extern crate lazy_static;
extern crate void;
extern crate polytype;

mod parser;
mod transformer;
mod typer;

use std::error::Error;
use from_pest::FromPest;
use pest::Parser;
use crate::parser::{ParseTree, Rule};
use crate::transformer::SyntaxTree;
use self::typer::TypedTree;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {

  let file = fs::read("../test.arc").or_else(|_| fs::read("test.arc"))?;

  let source = String::from_utf8(file)?;
  println!("source = {:#?}", source);

  let mut parse_tree = ParseTree::parse_program(&source)?;
  println!("parse tree = {:#?}\n", parse_tree);

  let syntax_tree = SyntaxTree::from_pest(&mut parse_tree).unwrap();
  println!("syntax tree = {:#?}\n", syntax_tree);

  let typed_tree = TypedTree::from(syntax_tree);
  println!("typed tree = {:#?}\n", typed_tree);

  Ok(())
}

#[test]
fn arc_example_runs() {
  main().unwrap()
}


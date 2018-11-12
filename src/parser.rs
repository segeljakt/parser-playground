#[derive(Parser)]
#[grammar = "arc.pest"]
pub struct ParseTree;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;


impl ParseTree {
  pub fn parse_program(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    ParseTree::parse(Rule::program, source)
  }
}

#![allow(bad_style)]
#![feature(box_syntax)]

#[macro_use]
extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;
#[macro_use]
extern crate lazy_static;
extern crate void;


mod arc {
  #[derive(Parser)]
  #[grammar = "arc.pest"]
  pub struct Parser;
}

mod ast {
  use super::arc::Rule;
  use pest::Span;
  use pest::prec_climber::PrecClimber;
  use pest::iterators::Pair;
  use pest::iterators::Pairs;

  fn span_into_str(span: Span) -> &str {
    span.as_str()
  }

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::program))]
  pub struct Program<'pest> ( Vec<Function<'pest>>, EOI,);
  
  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::function))]
  pub struct Function<'pest> (
    Ident<'pest>,
    Vec<Param<'pest>>,
    Expr,
  );

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::param))]
  pub struct Param<'pest> (
    Ident<'pest>,
  );

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::ident))]
  pub struct Ident<'pest> (
    #[pest_ast(outer(with(span_into_str)))]
    &'pest str
  );
  
  #[derive(Debug)]
  pub enum Expr {
    Not(Box<Not>),
    Neg(Box<Neg>),
    Add(Box<Add>),
    Sub(Box<Sub>),
    Mul(Box<Mul>),
    Div(Box<Div>),
    Pow(Box<Pow>),
    LitInt(Box<LitInt>),
    LitBool(Box<LitBool>),
  }

  #[derive(Debug)]
  pub struct Add(Expr, Expr);
  #[derive(Debug)]
  pub struct Sub(Expr, Expr);
  #[derive(Debug)]
  pub struct Mul(Expr, Expr);
  #[derive(Debug)]
  pub struct Div(Expr, Expr);
  #[derive(Debug)]
  pub struct Pow(Expr, Expr);
  #[derive(Debug)]
  pub struct Not(Expr);
  #[derive(Debug)]
  pub struct Neg(Expr);

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::lit_int))]
  pub struct LitInt(
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    i32
  );

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::lit_bool))]
  pub struct LitBool(
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    bool
  );

  #[derive(Debug, FromPest)]
  #[pest_ast(rule(Rule::EOI))]
  struct EOI;

  lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
      use pest::prec_climber::Assoc::*;
      use pest::prec_climber::Operator;
  
      PrecClimber::new(vec![
        Operator::new(Rule::add, Left) | Operator::new(Rule::sub, Left),
        Operator::new(Rule::mul, Left) | Operator::new(Rule::div, Left),
        Operator::new(Rule::pow, Right),
      ])
    };
  }

  use from_pest::FromPest;
  use from_pest::ConversionError;
  use void::Void;

  type ExprResult = Result<Expr, ConversionError<Void>>;
  impl<'pest> FromPest<'pest> for Expr {
    type Rule = Rule;
    type FatalError = Void;
    fn from_pest(pest: &mut Pairs<'pest, Rule>) -> ExprResult {
      PREC_CLIMBER.climb(pest,
        |pair: Pair<Rule>| Ok(match pair.as_rule() {
          Rule::lit_int  => Expr::LitInt(box LitInt::from_pest(&mut Pairs::single(pair))?),
          Rule::lit_bool => Expr::LitBool(box LitBool::from_pest(&mut Pairs::single(pair))?),
          Rule::not      => Expr::Not(box Not(Expr::from_pest(&mut pair.into_inner())?)),
          Rule::neg      => Expr::Neg(box Neg(Expr::from_pest(&mut pair.into_inner())?)),
          Rule::expr     => Expr::from_pest(&mut pair.into_inner())?,
          _ => unreachable!(),
        }),
        |lhs: ExprResult, op: Pair<Rule>, rhs: ExprResult| Ok(match op.as_rule() {
          Rule::add => Expr::Add(box Add(lhs?, rhs?)),
          Rule::sub => Expr::Sub(box Sub(lhs?, rhs?)),
          Rule::mul => Expr::Mul(box Mul(lhs?, rhs?)),
          Rule::div => Expr::Div(box Div(lhs?, rhs?)),
          Rule::pow => Expr::Pow(box Pow(lhs?, rhs?)),
          _ => unreachable!(),
        }),
      )
    }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  use self::ast::Program;
  use from_pest::FromPest;
  use pest::Parser;
  use std::fs;

  let source = String::from_utf8(fs::read("../test.arc").or_else(|_|fs::read("test.arc"))?)?;
  println!("{:#?}", source);
  let mut parse_tree = arc::Parser::parse(arc::Rule::program, &source)?;
  println!("parse tree = {:#?}\n", parse_tree);
  let syntax_tree: Program = Program::from_pest(&mut parse_tree).expect("lul");
  println!("syntax tree = {:#?}\n", syntax_tree);

  Ok(())
}

#[test]
fn arc_example_runs() {
  main().unwrap()
}


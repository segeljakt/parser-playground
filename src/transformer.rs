use crate::parser::Rule;
use pest::Span;
use pest::prec_climber::PrecClimber;
use pest::iterators::Pair;
use pest::iterators::Pairs;

fn span_into_str(span: Span) -> &str {
  span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct SyntaxTree<'a> (
  Expr<'a>,
  EOI
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function))]
pub struct Function<'a> (
  Ident<'a>,
  Vec<Param<'a>>,
  Expr<'a>,
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::param))]
pub struct Param<'a> (
  Ident<'a>,
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'a> (
  #[pest_ast(outer(with(span_into_str)))]
  &'a str
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expr))]
pub enum Expr<'a> {
  Let(Let<'a>),
  Op(Op<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::let_expr))]
pub struct Let<'a> (
  Binding<'a>,
  Box<Expr<'a>>,
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::binding))]
pub struct Binding<'a> (
  Ident<'a>,
  Op<'a>,
);


#[derive(Debug)]
pub enum Op<'a> {
  I32(i32),
  Bool(bool),
  Term(&'a str),
  Not(Box<Op<'a>>),
  Neg(Box<Op<'a>>),
  Add(Box<Op<'a>>, Box<Op<'a>>),
  Sub(Box<Op<'a>>, Box<Op<'a>>),
  Mul(Box<Op<'a>>, Box<Op<'a>>),
  Div(Box<Op<'a>>, Box<Op<'a>>),
  Pow(Box<Op<'a>>, Box<Op<'a>>),
}

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

type OpResult<'a> = Result<Op<'a>, ConversionError<Void>>;
impl<'a> FromPest<'a> for Op<'a> {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> OpResult<'a> {
    use self::Rule::*;
    use self::Op::*;
    PREC_CLIMBER.climb(pest,
      |pair: Pair<Rule>| Ok(
        match pair.as_rule() {
          lit_i32  => Op::I32(pair.as_str().parse().unwrap()),
          lit_bool => Op::Bool(pair.as_str().parse().unwrap()),
          term     => Op::Term(pair.as_str()),
          not      => Op::Not(box Op::from_pest(&mut pair.into_inner())?),
          neg      => Op::Neg(box Op::from_pest(&mut pair.into_inner())?),
          op       => Op::from_pest(&mut pair.into_inner())?,
          _ => unreachable!(),
        }),
      |lhs: OpResult, op: Pair<Rule>, rhs: OpResult| Ok(
        match op.as_rule() {
          add => Add(box lhs?, box rhs?),
          sub => Sub(box lhs?, box rhs?),
          mul => Mul(box lhs?, box rhs?),
          div => Div(box lhs?, box rhs?),
          pow => Pow(box lhs?, box rhs?),
          _ => unreachable!(),
        }))
  }
}

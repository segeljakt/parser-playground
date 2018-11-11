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
pub struct SyntaxTree<'a> {
  expr: Expr<'a>,
  eoi: EOI
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'a> (
  #[pest_ast(outer(with(span_into_str)))]
  &'a str
);

#[derive(Debug)]
pub enum Expr<'a> {
  LetExpr(Let<'a>),
  I32(i32),
  Bool(bool),
  Term(&'a str),
  Not(Box<Expr<'a>>),
  Neg(Box<Expr<'a>>),
  Add(Box<Expr<'a>>, Box<Expr<'a>>),
  Sub(Box<Expr<'a>>, Box<Expr<'a>>),
  Mul(Box<Expr<'a>>, Box<Expr<'a>>),
  Div(Box<Expr<'a>>, Box<Expr<'a>>),
  Pow(Box<Expr<'a>>, Box<Expr<'a>>),
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
  Box<Expr<'a>>,
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

type ExprResult<'a> = Result<Expr<'a>, ConversionError<Void>>;
impl<'a> FromPest<'a> for Expr<'a> {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> ExprResult<'a> {
    use self::Rule::*;
    //let pest = pest.next().unwrap().into_inner();
    println!("{:?}", pest);
    let res = PREC_CLIMBER.climb(pest,
      |pair: Pair<Rule>| Ok( {println!("{:?}\n", pair.as_rule());
        match pair.as_rule() {
          lit_i32  => Expr::I32(pair.as_str().parse().unwrap()),
          lit_bool => Expr::Bool(pair.as_str().parse().unwrap()),
          term     => Expr::Term(pair.as_str()),
          not      => Expr::Not(box Expr::from_pest(&mut pair.into_inner())?),
          neg      => Expr::Neg(box Expr::from_pest(&mut pair.into_inner())?),
          let_expr => Expr::LetExpr(Let::from_pest(&mut Pairs::single(pair))?),
          expr     => Expr::from_pest(&mut pair.into_inner())?,
          _ => unreachable!(),
        }}),
      |lhs: ExprResult, op: Pair<Rule>, rhs: ExprResult| Ok(
        match op.as_rule() {
          add => Expr::Add(box lhs?, box rhs?),
          sub => Expr::Sub(box lhs?, box rhs?),
          mul => Expr::Mul(box lhs?, box rhs?),
          div => Expr::Div(box lhs?, box rhs?),
          pow => Expr::Pow(box lhs?, box rhs?),
          _ => unreachable!(),
        }));
    println!("{:?}", res); res
  }
}

use crate::parser::Rule;
use pest::Span;
use pest::prec_climber::PrecClimber;
use pest::iterators::Pair;
use pest::iterators::Pairs;

use from_pest::FromPest;
use from_pest::ConversionError as PestError;
use void::Void;

fn span_into_str(span: Span) -> &str {
  span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct SyntaxTree<'a> (
  pub Expr<'a>,
  //EOI,
);
//#[derive(Debug, FromPest)]
//#[pest_ast(rule(Rule::EOI))]
//pub struct EOI;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'a> (
  #[pest_ast(outer(with(span_into_str)))]
  &'a str
);

#[derive(Debug)]
pub enum Expr<'a> {
  Let(Ident<'a>, Option<Type>, Box<Expr<'a>>, Box<Expr<'a>>),
  Lit(Value),
  Term(&'a str),
  Unary(UnOp, Box<Expr<'a>>),
  Binary(Box<Expr<'a>>, BinOp, Box<Expr<'a>>)
}

#[derive(Debug)]
pub enum Value {
  I32(i32),
  Bool(bool)
}

impl<'a> FromPest<'a> for Value {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<Value, PestError<Void>> {
    use self::{Rule::*, Value::*};
    let pair = pest.next().unwrap();
    match pair.as_rule() {
      lit_i32  => Ok(I32(pair.as_str().parse().unwrap())),
      lit_bool => Ok(Bool(pair.as_str().parse().unwrap())),
      _        => unreachable!(),
    }
  }
}


#[derive(Debug)]
pub enum UnOp {
  Not,
  Neg,
}

impl<'a> FromPest<'a> for UnOp {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<UnOp, PestError<Void>> {
    use self::{Rule::*, UnOp::*};
    match pest.next().unwrap().as_rule() {
      not => Ok(Not),
      neg => Ok(Neg),
      _   => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
  Pow,
}

impl<'a> FromPest<'a> for BinOp {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<BinOp, PestError<Void>> {
    use self::{Rule::*, BinOp::*};
    match pest.next().unwrap().as_rule() {
      add => Ok(Add),
      sub => Ok(Sub),
      mul => Ok(Mul),
      div => Ok(Div),
      pow => Ok(Pow),
      _   => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub enum Type {
  I32,
  Bool,
  Appender(Box<Type>),
  Merger(BinOp),
}

impl<'a> FromPest<'a> for Type {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<Type, PestError<Void>> {
    use self::{Rule::*, Type::*};
    if let Some(pair) = pest.next() {
      match pair.as_rule() {
        s_i32      => Ok(I32),
        s_bool     => Ok(Bool),
        b_appender => Ok(Appender(box Type::from_pest(&mut pair.into_inner())?)),
        b_merger   => Ok(Merger(BinOp::from_pest(&mut pair.into_inner())?)),
        _          => unreachable!(),
      }
    } else {
      Err(PestError::NoMatch)
    }
  }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::binding))]
pub struct Binding<'a> (
  Symbol<'a>,
  Expr<'a>,
);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::symbol))]
struct Symbol<'a> (
  Ident<'a>,
  Option<Type>,
);

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

type ExprResult<'a> = Result<Expr<'a>, PestError<Void>>;
impl<'a> FromPest<'a> for Expr<'a> {
  type Rule = Rule;
  type FatalError = Void;
  fn from_pest(pest: &mut Pairs<'a, Rule>) -> ExprResult<'a> {
    use self::{Rule::*, Expr::*, UnOp::*, BinOp::*, Value::*};
    PREC_CLIMBER.climb(pest,
      |pair: Pair<Rule>| Ok(
        match pair.as_rule() {
          expr     => Expr::from_pest(&mut pair.into_inner())?,
          term     => Term(pair.as_str()),
          lit      => Lit(Value::from_pest(&mut pair.into_inner())?),
          unary    => {
            let mut i = pair.into_inner();
            let u = UnOp::from_pest(&mut Pairs::single(i.next().unwrap()))?;
            let e = Expr::from_pest(&mut Pairs::single(i.next().unwrap()))?;
            Unary(u, box e)
          }
          let_expr => {
            let mut i = pair.into_inner();
            let b = Binding::from_pest(&mut Pairs::single(i.next().unwrap()))?;
            let e = Expr::from_pest(&mut i.next().unwrap().into_inner())?;
            Let((b.0).0, (b.0).1, box b.1, box e)
          },
          _        => unreachable!(),
        }),
      |lhs: ExprResult, op: Pair<Rule>, rhs: ExprResult| Ok(
        Binary(box lhs?, BinOp::from_pest(&mut Pairs::single(op))?, box rhs?)
      ))
  }
}

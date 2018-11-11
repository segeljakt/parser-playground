use polytype::*;
pub struct TypedTree<'a> (
  TypedExpr<'a>
);

use crate::transformer::SyntaxTree;

impl<'a> TypedTree<'a> {
  pub fn new(syntax_tree: SyntaxTree) {
    let mut ctx = Context::default();
    // let x: i32 = 3; let y = x + 5;
    let x = Type::Constructed("i32", vec![]);
    let y = ctx.new_variable();
    ctx.unify(&x, &y).expect("unifies");
    let x = x.apply(&ctx);
    let y = y.apply(&ctx);
    assert_eq!(x, y);

    println!("{:#?}", x);
    println!("{:#?}", y);
    println!("{:#?}", ctx);
  }
}

pub enum TypedExpr<'a> {
  TypedLet(TypedLet<'a>),
  TypedOp(TypedOp<'a>),
}

pub struct TypedLet<'a> (
  TypedBinding<'a>,
  Box<TypedExpr<'a>>,
);

pub struct TypedBinding<'a> (
  TypedIdent<'a>,
  TypedOp<'a>,
);

pub enum TypedOp<'a> {
  I32(i32),
  Bool(bool),
  Term(&'a str),
  Not(Box<TypedOp<'a>>),
  Neg(Box<TypedOp<'a>>),
  Add(Box<(TypedOp<'a>, TypedOp<'a>)>),
  Sub(Box<(TypedOp<'a>, TypedOp<'a>)>),
  Mul(Box<(TypedOp<'a>, TypedOp<'a>)>),
  Div(Box<(TypedOp<'a>, TypedOp<'a>)>),
  Pow(Box<(TypedOp<'a>, TypedOp<'a>)>),
}

pub struct TypedIdent<'a> (
  &'a str
);


use polytype::*;

#[derive(Debug)]
pub struct TypedTree<'a> (
  TypedExpr<'a>
);

use crate::transformer::*;

impl<'a> From<SyntaxTree<'a>> for TypedTree<'a> {
  fn from(x: SyntaxTree<'a>) -> TypedTree<'a> {
    //let mut ctx = Context::default();
    // let x: i32 = 3; let y = x + 5;
    //let x = Type::Constructed("i32", vec![]);
    //let y = ctx.new_variable();
    //ctx.unify(&x, &y).expect("unifies");
    //let x = x.apply(&ctx);
    //let y = y.apply(&ctx);
    //assert_eq!(x, y);

    //println!("{:#?}", x);
    //println!("{:#?}", y);
    //println!("{:#?}", ctx);
    unreachable!();
    //TypedTree (
      //TypedExpr::from(expr)
    //)
  }
}

#[derive(Debug)]
pub enum TypedExpr<'a> {
  //TypedLet(Symbol, Box<TypedExpr<'a>>, Box<TypedExpr<'a>>),
  //TypedLit(Type, Value),
  TypedTerm(&'a str),
  //TypedUnary(UnOp, Box<TypedExpr<'a>>),
  //TypedBinary(Box<TypedExpr<'a>>, BinOp, Box<TypedExpr<'a>>)
}

//impl<'a> From<Expr<'a>> for TypedExpr<'a> {
  //fn from(expr: Expr<'a>) -> Self {
    //use crate::transformer::{Expr::*};
    //match expr {
      //Let(sym) => ,
      //Lit(Value()) => ,
      //Binary(lhs,op,rhs) => ,
      //Unary(lhs,op,rhs) => ,
      //_ => unreachable!()
    //}
  //}
//}

//#[derive(Debug)]
//struct Symbol;

//enum Type {
  //I32,
  //Bool,
//}

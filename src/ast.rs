use std::fmt::Display;

use serde::Serialize;

use crate::token::Token;

#[derive(Debug, Serialize)]
pub enum Statement {
   SetAssign { name: Token, value: Expression },
   VarAssign { name: Token, value: Expression },
   IfStatement { boolean: Expression, block: Vec<Statement> },
   FunCall { name: Token, args: Vec<Expression> },
   FunDef { name: Token, args: Vec<Expression>, yeild: Expression, block: Vec<Statement> },
   Void
}

impl Display for Statement {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
   }
}

#[derive(Debug, Serialize)]
pub enum Expression {
   Literal { value: Token },
   Array { value: Vec<Self> },
   BooleanExpr { lhs: Box<Expression>, rhs: Box<Expression>, operator: Token },
   FunCall { name: Token, args: Vec<Expression> },
   Type { value: Token, is_array: usize },
   Argument { name: Token, class: Box<Expression> },
   Null,
}

impl Display for Expression {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
   }
}
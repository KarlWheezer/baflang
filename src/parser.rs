use crate::ast::{Expression, Statement};
use crate::token::Class;
use crate::{lexer::Lexer, token::Token};
use crate::util::{Colorize, Split};


const SYNTAX: &[&[&str]] = &[
   &["fun", "{name}", "(", "{args}", ")", "->", "{yeild-type}", "[", "...", "]"],
   &["if", "{boolean-expr}", "[", "...", "]"],
   
   &["set", "{name}", "=", "{expr}", ";"],
   &["var", "{name}", "=", "{expr}", ";"],

   &["use", "{pacakge}", ";"],
   &["use", "{pacakge}", ":", "{value}", ";"],
   &["use", "{pacakge}", "as", "{alias}", ";"],

   &["[", "{statements}", "]"],
   //: EXPRESSIONS ://
   &["{string}"], &["{number}"], &["{identifier}"],
   &["[", "{expression}", "]"],
   &["{expression}", "{comparator}", "{expression}"],
   &["(", "{arguments}", ")"],
   &["{name}", "(", "{arguments}", ")"],
];

pub struct Parser {
   tokens: Vec<Token>, //: token array
   index: usize, //: for indexing
   filename: String, //: for errors
   source: Vec<String>, //: for errors
}

impl Parser {
   pub fn init(lexer: Lexer) -> Self {
      let filename = lexer.filename.clone();
      let source = lexer.source.split('\n');
      Parser { 
         tokens: lexer.tokenize(), 
         index: 0, filename, source,
      }
   }

   fn nth(&self, offset: isize) -> Token {
      let index = self.index as isize + offset;
      if index < self.tokens.len() as isize && index >= 0
         { self.tokens[index as usize].clone() }
      else
         { self.tokens[self.tokens.len()-1].clone() }
   }
   fn cur(&self) -> Token {
      self.nth(0)
   }
   fn next(&mut self) {
      self.index += 1;
   } 

   fn area(&self) -> String {
      format!("{}{}", self.filename, self.cur().index()).colorize(4)
   }
   fn fmt(&self, index: [usize; 2]) -> String {
      let mut vec = vec![];

      for i in 0..SYNTAX[index[0]].len() {
         if i == index[1]
            { vec.push(SYNTAX[index[0]][i].colorize(7)); }
         else
            { vec.push(SYNTAX[index[0]][i].to_string()); }
      }
      return vec.join(" ");
   }
   fn error(&self, info: &str, syntax: String) {
      let mut buf = String::new();
      let mut line = String::new();
      let cur = self.cur();

      for _ in 0..cur.index[0].to_string().len()
         { line.push(' '); } buf.push_str("| ");

      buf = line.clone();

      for _ in 0..cur.index[0].to_string().len()
         { buf.push(' '); } buf.push_str("| ");
      for _ in 1..cur.index[1] 
         { buf.push(' '); } buf.push('^');
      for _ in 1..cur.value.len()
         { buf.push('~'); } 

      println!("{}", vec![
         format!("{}:{} --> {}", "error".colorize(31), "parser".colorize(36), self.area()),
         format!("{} | {}", self.cur().index[0], self.source[self.cur().index[0]-1]), 
         format!("{buf} {info}"),
         format!("{line} | {}: {syntax}", "info".colorize(34)),
      ].join("\n"));
   }
   fn eat(&mut self, class: Class, syntax: [usize; 2]) -> Token {
      let cur = self.nth(0);

      if cur.class != class 
         { self.error(&format!("expected {class}, got {}", cur.class), self.fmt(syntax)); }

      self.next(); return cur;
   }

   fn collect_expression(&mut self, parent: [usize; 2]) -> Expression {
      let cur = self.cur();
      let value = match cur.class {
         Class::LeftBrace => self.collect_array(parent),

         Class::String => { self.next(); Expression::Literal { value: cur } },
         Class::Number => { self.next(); Expression::Literal { value: cur } },
         Class::Boolean => { self.next(); Expression::Literal { value: cur } },

         Class::Identifier => { match self.nth(1).class {
            Class::LeftParen => self.collect_fun_call(parent),
            _ => { self.error("expected '(' after identifier", self.fmt([14, 1])); Expression::Null }
         } }

         _ => {
            self.error(&format!("expected expression, found {}", cur.class), self.fmt(parent));

            Expression::Null
         }
      }; 
      
      if self.cur().class == Class::Comparator {
         return self.collect_boolean_expression(value);
      }
      
      return value;
   }
   fn collect_array(&mut self, parent: [usize; 2]) -> Expression {
      self.next(); let mut value = vec![];
      let mut comma = true;

      loop {
         let cur = self.cur();

         match cur.class {
            Class::Eof => {
               self.error("expected ']' to finish array parsing but foudn '\\0'", self.fmt([11,2]));
               return Expression::Null;
            },
            Class::RightBrace => {
               self.next(); break;
            },
            Class::Comma => { comma = true; self.next(); },
            _ if comma => {
               comma = false;
               value.push(self.collect_expression(parent));
            },
            _ => {
               self.error(&format!("expected ',' before another expression but found {}", cur.class), self.fmt([11, 1]));
               return Expression::Null;
            }
         }
      }

      return Expression::Array { value }
   }
   fn collect_boolean_expression(&mut self, lhs: Expression) -> Expression {
      let operator = self.eat(Class::Comparator, [12, 1]);
      let rhs = self.collect_expression([12, 2]);

      Expression::BooleanExpr { lhs: Box::from(lhs), rhs: Box::from(rhs), operator }
   }
   fn collect_arguments(&mut self, parent: [usize; 2]) -> Vec<Expression> {
      let mut args = vec![]; 
      let mut comma = true;


      self.next(); loop {
         let cur = self.cur();

         match cur.class {
            Class::Eof => {
               self.error("expected ']' to finish array parsing but foudn '\\0'", self.fmt([13,2]));
               return vec![];
            },
            Class::RightParen => {
               self.next(); break;
            },
            Class::Comma => { comma = true; self.next(); },
            _ if comma => {
               comma = false;
               args.push(self.collect_expression(parent));
            },
            _ => {
               self.error(&format!("expected ',' before another expression but found {}", cur.class), self.fmt([13, 1]));
               return vec![];
            }
         }
      }

      return args;
   }
   fn collect_fun_call(&mut self, parent: [usize; 2]) -> Expression {
      let name = self.eat(Class::Identifier, [14, 0]);
      let args = self.collect_arguments(parent);

      return Expression::FunCall { name, args }
   }

   pub fn scan_for_statements(&mut self) -> Vec<Statement> {
      let mut statements = vec![];
      while self.cur().class != Class::Eof {
         statements.push(self.scan_for_statement());
      }

      return statements;
   }
   fn scan_for_statement(&mut self) -> Statement {
      let cur = self.cur();
      let statement: Statement = match cur.class {
         Class::Keyword => match cur.value.as_str() {
            "set" => self.scan_set_assign(),
            "var" => self.scan_var_assign(),
            "if" => self.scan_for_if(),
            _ => {
               todo!("not yet implimented - {}", cur)
            },
         },
         _ => Statement::Void,
      };

      return statement;
   }

   fn scan_code_block(&mut self) -> Vec<Statement> {
      self.eat(Class::LeftBrace, [12, 0]);
      let mut statements = vec![];
      while self.cur().class != Class::RightBrace {
         if self.cur().class == Class::Eof {
            self.error("unexpected end-of-file, wanted ']' for closing code block", self.fmt([12,0]));
            break;
         }

         statements.push(self.scan_for_statement());
      }; 
      
      self.next(); return statements;
   }
   fn scan_set_assign(&mut self) -> Statement {
      self.next();
      let name = self.eat(Class::Identifier, [2, 1]);
      self.eat(Class::Assign, [2, 2]);
      let value = self.collect_expression([2, 3]);
      self.eat(Class::Semi, [2, 4]);

      Statement::SetAssign { name, value }
   }
   fn scan_var_assign(&mut self) -> Statement {
      self.next();
      let name = self.eat(Class::Identifier, [2, 1]);
      self.eat(Class::Assign, [2, 2]);
      let value = self.collect_expression([2, 3]);
      self.eat(Class::Semi, [2, 4]);

      Statement::VarAssign { name, value }
   }

   fn scan_for_if(&mut self) -> Statement {
      self.next();
      let boolean = self.collect_expression([1,1]);
      let block = self.scan_code_block();

      Statement::IfStatement { boolean, block }
   }
}
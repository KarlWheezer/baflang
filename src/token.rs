use std::fmt::Display;
use lorust::kebab_case;
use serde::Serialize;
use crate::util::Colorize;

#[derive(Clone, Debug)]
pub struct Token {
   pub class: Class,
   pub value: String,
   pub index: [usize; 2]
}
impl Token {
   pub fn new(class: Class, value: String, index: [usize; 2]) -> Self {
      Self { class, value, index }
   }
   pub fn index(&self) -> String {
      return format!("[{}:{}]", self.index[0], self.index[1]).colorize(1);
   }
   pub fn to_string(&self) -> String {
      format!("{{ class: {}, value: {}, index: {:?} }}", self.class, self.value, self.index)
   }
}
impl Display for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "token|{}, '{}', [{}:{}]|", 
         self.class.colorize(33), self.value.colorize(32), 
         self.index[0].colorize(36), self.index[1].colorize(36)
      )
   }
}

impl Serialize for Token {
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
   where S: serde::Serializer {
      serializer.serialize_str(&self.to_string())
   }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum Class {
   Identifier, Keyword,
   String, Number, Boolean,

   LeftBrace, RightBrace,
   LeftBrack, RightBrack,
   LeftParen, RightParen,

   Dot, Comma, Semi, Colon,
   Operator, Comparator, 
   Assign, Arrow, Logic, Eof,
   Bang,
}

impl Display for Class {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", kebab_case(format!("{:?}", &self)))
   }
}
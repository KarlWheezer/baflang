use crate::token::{Class, Token};

pub trait Colorize<T> {
   fn colorize(&self, num: u8) -> String;
}

impl<T> Colorize<T> for T where T:ToString {
   fn colorize(&self, num: u8) -> String {
      return format!("\x1b[{}m{}\x1b[0m", num, self.to_string());
   }
}

pub trait Is<X> {
   fn is(&self, rhs: X) -> bool;
}

impl Is<(&str, Class)> for Token {
   fn is(&self, rhs: (&str, Class)) -> bool {
      self.class == rhs.1 && &self.value == rhs.0
   }
}
impl Is<&str> for Token {
   fn is(&self, rhs: &str) -> bool {
      &self.value == rhs
   }
}
impl Is<Class> for Token {
   fn is(&self, rhs: Class) -> bool {
      self.class == rhs
   }
}

pub trait Split {
   fn split(&self, seq: char) -> Vec<String>;
}

impl<Y> Split for Y where Y:ToString {
   fn split(&self, seq: char) -> Vec<String> {
      let mut array: Vec<String> = vec![];
      let mut value = String::new();
      let self_as_string: Vec<char> = self.to_string().chars().collect();

      for i in 0..self_as_string.len() {
         if self_as_string[i] == seq
            { array.push(value); value = String::new(); }
         else 
            { value.push(self_as_string[i]); }
      } 
      
      array.push(value); return array;
   }
}
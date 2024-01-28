use crate::token::{Class, Token};

const LERRORS: [&str; 6] = [
   "Invalid character found",
   "Unexpected EOF",
   "Unterminated string or comment",
   "Invalid numerical format",
   "Unclosed brackets",
   "Invalid escape sequence"
];

pub struct Lexer {
   index: usize,
   coord: [usize; 2],

   pub filename: String,
   pub source: String,
   tokens: Vec<Token>,
   errors: [&'static str; 6],
}

impl Lexer {
   pub fn init() -> Self {
      let [filename, source] = Self::read_args();
      Self { 
         index: 0, coord: [1, 1], 
         filename, source, 
         tokens: vec![], errors: LERRORS,
      }
   }
   fn read_args() -> [String; 2]{
      let name = std::env::args().nth(1);
      let filename = if name.is_some() {
         name.unwrap()
      } else {
         println!("error[S0]: no filename provided -> stdin[1:1]");
         std::process::exit(-1);
      };

      let code = std::fs::read_to_string(&filename);
      let source = if code.is_ok() {
         code.unwrap()
      } else {
         println!("error[S1]: invalid filename provided -> stdin[1:2]");
         std::process::exit(-1);
      };

      [filename, source]
   }

   fn char(&self) -> char {
      if self.index < self.source.len() 
         { self.source.as_bytes()[self.index] as char } else { '\0' }
   }
   fn next(&mut self) {
      if self.char() == '\n'
         { self.coord[0] += 1; self.coord[1] = 1; }
         self.coord[1] += 1;
         self.index += 1;
   }
   fn push(&mut self, class: Class, value: String, index: [usize; 2]) {
      self.tokens.push(Token::new(class, value, index));
   }
   fn stack(&mut self, class: Class) {
      self.push(class, self.char().to_string(), self.coord);
      self.next();
   }
   fn info(&self) -> String {
      format!("{}[{}:{}]", self.filename, self.coord[0], self.coord[1])
   }
   fn error(&self, code: usize, info: &str) {
      println!("error[L00{}]: {} {} {}", code + 1, self.errors[code], info, self.info())
   }

   fn scan(&mut self) {
      let cur = self.char();
      match cur {
         _ if cur.is_alphabetic() => {
            let mut value = String::new();
            let index = self.coord.clone();

            while self.char().is_alphanumeric() 
               { value.push(self.char()); self.next(); }
            
            let class = match value.as_str() {
               "fun" | "set" | "var" | "if" |
               "use" | "yeild" => Class::Keyword,
               "true" | "false" => Class::Boolean,
               _ => Class::Identifier
            };

            self.push(class, value, index);
         },
         _ if cur.is_numeric() => {
            let mut value = String::new();
            let index = self.coord.clone();
            let mut dots: u8 = 0;

            while self.char().is_numeric() || self.char() == '.' {
               if self.char() == '.' {
                  dots += 1;
                  if dots < 1 { value.push('.'); self.next(); }
                  else {
                     self.error(3, "too many '.' found"); self.next();
                  }
               }
               value.push(self.char());
               self.next();
            }

            self.push(Class::Number, value, index);
         },
         
         '"' => {
            let mut value = String::new();
            let index = self.coord.clone();
        
            self.next(); // Skip initial '"'
            while self.char() != '"' {
               if self.char() == '\\' {
                  self.next(); // Skip the backslash
                  match self.char() {
                     'n' => value.push('\n'),
                     't' => value.push('\t'),
                     '\\' => value.push('\\'),
                     '"' => value.push('"'),
                     _ => {
                        self.error(5, &format!("'\\{}' is not valid", self.char()));
                     }
                  }
               } else if self.char() == '\0' {
                  self.error(2, "");
                  std::process::exit(1);
               } else {
                  value.push(self.char());
               }
               self.next();
            }
            self.next(); self.push(Class::String, value, index);
         },

         '[' => self.stack(Class::LeftBrace),
         '{' => self.stack(Class::LeftBrack),
         '(' => self.stack(Class::LeftParen),

         ']' => self.stack(Class::RightBrace),
         '}' => self.stack(Class::RightBrack),
         ')' => self.stack(Class::RightParen),

         '.' => self.stack(Class::Dot),
         ',' => self.stack(Class::Comma),
         ':' => self.stack(Class::Colon),
         ';' => self.stack(Class::Semi),

         '+' | '*' | '/' => {
            self.stack(Class::Operator);
         }

         '<' | '>' => {
            let mut value = self.char().to_string();
            let index = self.coord; self.next();

            if self.char() == '=' {
               self.next(); value.push('=');
            }

            self.push(Class::Comparator, value, index);
         },

         '-' => {
            let index = self.coord.clone();
            self.next();
            if self.char() == '>' {
               self.push(Class::Arrow, "->".to_string(), index); self.next();
            } else {
               self.push(Class::Operator, "-".to_string(), index);
            }
         },

         '&' | '|' => self.stack(Class::Logic),

         '!' => {
            let index = self.coord; self.next();
            if self.char() == '=' {
               self.push(Class::Comparator, "!=".to_string(), index); self.next();
            } else {
               self.push(Class::Bang, "!".to_string(), index);
            }
         },

         '=' => {
            let index = self.coord;
            self.next();
            if self.char() == '=' {
               self.push(Class::Comparator, "==".to_string(), index);
               self.next();
            } else {
               self.push(Class::Assign, "=".to_string(), index)
            }
         }

         ' ' | '\n' => self.next(),

         _ => {
            self.error(0, &format!("{:?}", self.char()));self.next();
         }
      };
   }
   pub fn tokenize(mut self) -> Vec<Token> {
      while self.char() != '\0' && self.index < self.source.len()
         { self.scan(); } self.stack(Class::Eof);
      
      return self.tokens;
   }
}
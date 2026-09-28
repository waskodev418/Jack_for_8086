use std::iter::Peekable;
use std::str::Chars;
pub struct Tokenizer;

impl Tokenizer {

  fn get_keyword(token: &str) -> String {

    match token {
      "class" | "constructor" | "function" | "method" | 
      "field" | "static" | "var" | "int" | "char" | 
      "boolean" | "void" | "true" | "false" | "null" | 
      "this" | "let" | "do" | "if" | "else" | "while" | 
      "return" => format!("<keyword> {} </keyword>", token),

      _ => format!("<identifier> {} </identifier>", token),
    }
  }

  fn is_symbol(el: char) -> bool {
   match el {
    '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' 
     | ';' | '+' | '-' | '*' | '|' | '=' | 
     '~' | '/' | '<' | '>' | '&' => true,
     _ => false
   }
  }

  fn get_word_until(stream: &mut Peekable<Chars<'_>>, delim: &str, strict: bool) -> String {
   let mut word = String::new();
   let mut done = false;

   while let Some(&ch) = stream.peek() {
     let s = ch.to_string();
     if s == delim {
       stream.next();
       done = true;
       break;
     } else if !strict && (Tokenizer::is_symbol(ch)) {
      done = true;
      break;
     }
     stream.next();
     word.push(ch);
   }

   if done {
    word
   } else {
    "".to_string()
   }
  } 

  fn get_number(first: char, stream: &mut Peekable<Chars<'_>>) -> String {
   let mut number = String::from(first);

   while let Some(&ch) = stream.peek() {
     if !ch.is_ascii_digit() {
       break; 
     }
     number.push(ch);
     stream.next(); 
   }

   match number.parse::<u16>() {
     Ok(val) if val <= 32767 => format!("<integerConstant> {} </integerConstant>", number),
     _ => panic!("numeric overflow: {}", number),
   }
  }

 pub fn remove_comment(stream: &mut Peekable<Chars<'_>>) -> String {

  match stream.peek() {
    Some('/') => {
      stream.by_ref().for_each(|_| ());
      "".to_string()
    }
    Some('*') => {
      stream.next();
      Tokenizer::get_word_until(stream, "*/", true);
      "".to_string()
    }
    _ => {
      format!("<symbol> / </symbol>")
    }
  }
 }

  pub fn get_token(line: &str) -> String {
   let mut result_set = String::new();
   let mut lexicon: std::iter::Peekable<std::str::Chars<'_>> = line.chars().peekable();
   
   while !lexicon.peek().is_none() {
     let first = lexicon.next().unwrap();
     let res = match first {
     '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' 
      | ';' | '+' | '-' | '*' | '|' | '=' | 
      '~' => format!("<symbol> {} </symbol>", first),

      '/' => Tokenizer::remove_comment(&mut lexicon),
      '<' => "<symbol> < </symbol>".to_string(),
      '>' => "<symbol> > </symbol>".to_string(),
      '&' => "<symbol> & </symbol>".to_string(),
      '"' =>{
       let t = Tokenizer::get_word_until(&mut lexicon, "\"", true);
       if t.is_empty() {
        "".to_string()
       } else{
        format!("<stringConstant> {} </stringConstant>",t)
       }
      },
      ' ' => String::from(""),
      _ => String::from("V")
     };

     if res == "V" {
      if first.is_numeric() {
       result_set.push_str(&Tokenizer::get_number(first, &mut lexicon));
      } else {
       let keyword = format!("{0}{1}",first,Tokenizer::get_word_until(&mut lexicon, " ", false)); 
       if keyword.is_empty() {
        panic!("no token found")
       }       
       result_set.push_str(&Tokenizer::get_keyword(&keyword));
      }
     } else if !res.is_empty() {
      result_set.push_str(&res);
     }

     if !res.is_empty() {
      result_set.push_str("\n");
     }
   }

   result_set.trim().to_string()
  }
}
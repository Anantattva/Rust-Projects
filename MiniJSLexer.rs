// ।। ॐ नमः शिवाय ।। \\
// ॥ ॐ वामदेवाय नमः ॥ \\

// ++ Mini JavaScript Lexer ++ \\

/*
 * @date
 * START: 22nd August, 2026
 * END: 22nd August, 2026
 */

use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

///// ============================ \\\\\
///// ++++++++++ SETUP ++++++++++ \\\\\
// ++ Structs Layout &Implementations Declaration ++ \\
///// ============================ \\\\\

#[derive(Debug, Default)] // Default trait helps default initialization of Structs, much like ::new() method;
pub struct Token {
  token_type: String,
  value: String
}

impl Token {
  // << constructor method >> \\
  pub fn new(token_type: String, value: String) -> Self {
    Token { token_type, value }
  }
}

#[derive(Debug, Default)]
pub struct TokensList(Vec<Token>);

fn get_identifiers() -> HashMap<&'static str, &'static str> {
  // In Rust, &'static str represents a string slice that lives for the entire duration of the program.
  // While using String works, &'static str is preferred for fixed compile-time lookups because it is significantly faster and uses less memory.
  let mut keywords = HashMap::new();

  // << INSERT KEYWORDS >> \\
  
  keywords.insert("let", "KEYWORD_DECLARATION");
  keywords.insert("function", "KEYWORD_FUNCTION");
  keywords.insert("return", "KEYWORD_RETURN");
  
  keywords.insert("if", "KEYWORD_IF");
  keywords.insert("else", "KEYWORD_ELSE");
  keywords.insert("else if", "KEYWORD_ELSEIF");
  
  keywords.insert("for", "KEYWORD_FORLOOP");
  keywords.insert("while", "KEYWORD_WHILELOOP");
  
  keywords.insert("console", "KEYWORD_CONSOLE");
  keywords.insert("log", "KEYWORD_LOG");
  keywords.insert("warn", "KEYWORD_WARN");
  keywords.insert("new", "KEYWORD_NEW");
  keywords.insert("throw", "KEYWORD_THROW");
  keywords.insert("error", "KEYWORD_ERROR");

  keywords
}

fn tokenize_delimiters(char: char, tokens: &mut TokensList) {
  match char {
    '\'' => tokens.0.push(Token { token_type: "SINGLE_QUOTE".to_string(), value: char.to_string() }),
    // r'\"' => tokens.0.push(Token { token_type: "DOUBLE_QUOTE".to_string(), value: char.to_string() }),
    // r'`' => tokens.0.push(Token { token_type: "TEMPLATE_LITERAL".to_string(), value: char.to_string() }),
    
    '=' => tokens.0.push(Token { token_type: "ASSIGNMENT".to_string(), value: char.to_string() }),
    
    '(' => tokens.0.push(Token { token_type: "LPAREN".to_string(), value: char.to_string() }),
    ')' => tokens.0.push(Token { token_type: "RPAREN".to_string(), value: char.to_string() }),
      
    '{' => tokens.0.push(Token { token_type: "LBRACE".to_string(), value: char.to_string() }),
    '}' => tokens.0.push(Token { token_type: "RBRACE".to_string(), value: char.to_string() }),
      
    '[' => tokens.0.push(Token { token_type: "LSQUARE".to_string(), value: char.to_string() }),
    ']' => tokens.0.push(Token { token_type: "RSQUARE".to_string(), value: char.to_string() }),
      
    '<' => tokens.0.push(Token { token_type: "LANGLE".to_string(), value: char.to_string() }),
    '>' => tokens.0.push(Token { token_type: "RANGLE".to_string(), value: char.to_string() }),
      
    '\\' => tokens.0.push(Token { token_type: "BACKSLASH".to_string(), value: char.to_string() }),
    '/' => tokens.0.push(Token { token_type: "FORWARDSLASH".to_string(), value: char.to_string() }),
      
    '.' => tokens.0.push(Token { token_type: "DOT".to_string(), value: char.to_string() }),
    ':' => tokens.0.push(Token { token_type: "COLON".to_string(), value: char.to_string() }),
    ';' => tokens.0.push(Token { token_type: "SEMICOLON".to_string(), value: char.to_string() }),
    '!' => tokens.0.push(Token { token_type: "EXCLAMATION".to_string(), value: char.to_string() }),
    '?' => tokens.0.push(Token { token_type: "TERNARY".to_string(), value: char.to_string() }),
    _ => {},
  }  
}

fn tokenize_keywords(code: String, tokens: &mut TokensList) {
  let keywords = get_identifiers();
  
  let mut chars = code.char_indices().peekable(); // .peekable() turned Iterator into peekable Iterator, allowing one to look to next item without accidentally consuming it;
  // In Rust, = is used for Pattern Matching, while == is used for Boolean Equality.
  // while let is a specialized control flow structure. It tells Rust: "Keep running this loop as long as the value on the right matches the pattern on the left, and extract (destructure) the inner data."
  // Here is what happens step-by-step on every iteration:
  // Evaluate Right Side: chars.peek() runs and returns an Option.
  // Match Pattern Left Side: Rust checks: "Is this Option a Some variant?"
  // If Yes (Some): It destructures the tuple inside, assigns the character to next_char, and runs the loop block.
  // If No (None): The pattern fails, the loop breaks immediately, and execution continues after the loop.
  while let Some((_, char)) = chars.next() {
    // << skip whitespaces >> \\
    if char.is_whitespace() {
      continue;
    }

    // << check keywords >> \\
    if char.is_alphabetic() || char == '_' {
      let mut word = String::new();
      word.push(char);
      
      // << keep consuming until word ends >> \\
      while let Some((_, next_char)) = chars.peek() {
        if next_char.is_alphanumeric() || char == '_' {
          word.push(*next_char);
          chars.next(); // consume it;
        } else {
          break;
        }
      }

      // << check type match >> \\
      let token_type = match keywords.get(word.as_str()) {
        Some(&kw_type) => kw_type.to_string(),
        None => "IDENTIFIER".to_string(),
      };
      tokens.0.push(Token::new(token_type, word));
      continue;
    }
  
  // << delimiters & single characters >> \\
  tokenize_delimiters(char, tokens);
  }
}

///// ========================== \\\\\
// ++ UNIFIED MASTER FUNCTION ++ \\
///// ========================== \\\\\
fn tokenize(code: String) -> TokensList {
  let start = Instant::now();
  let mut tokens: TokensList = Default::default();
  tokenize_keywords(code, &mut tokens);
  println!("Tokens List: {:#?}.", tokens);
  let end = Instant::now();
  let time = (end - start).as_micros() as f32;
  let time2 = (end - start).as_millis() as f32;
  println!("Time taken for one lexical tokenization: {}mcs OR {:3}ms.", time, time2);
  tokens
}

fn main() {
  println!("Building JS Lexer!!");
  let code1 = String::from("let age = 19; let name = 'karan'; function add() {}");
  tokenize(code1); // took less than 400 microseconds;
}
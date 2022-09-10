use std::fmt;

#[derive(Debug)]
pub enum Token {
  Id(String),    // x
  Num(usize),    // 1
  Str(String),   // "x"
  LPar,          // (
  RPar,          // )
  Dot,           // .
  Colon,         // :
  Pipe,          // |
  Eq,            // =

  // Keywords

  In,            // in
  Let,           // let
  Type,          // type
  Eof,           // eof

  // Special ones (Normally unicode)

  Lambda,        // λ
  Arrow,         // →
  Star,          // ★
  Pi,            // Π
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Token::*;

    match self {
      Id(string) => write!(f, "Id '{}'", string),
      Num(num) => write!(f, "Num '{}'", num),
      Str(string) => write!(f, "Str '{}'", string),
      LPar => write!(f, "'('"),
      RPar => write!(f, "')'"),
      Dot => write!(f, "'.'"),
      Colon => write!(f, "':'"),
      Pipe => write!(f, "'|'"),
      Eq => write!(f, "'='"),
      In => write!(f, "'in'"),
      Let => write!(f, "'let'"),
      Type => write!(f, "'type'"),
      Eof => write!(f, "'eof'"),
      Lambda => write!(f, "'λ'"),
      Arrow => write!(f, "'→'"),
      Star => write!(f, "'★'"),
      Pi => write!(f, "'Π'"),
    }
  }
}
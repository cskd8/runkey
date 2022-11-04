use phf::phf_map;

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: String,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

pub const KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};

pub fn lookup_ident(ident: String) -> String {
    match KEYWORDS.get(&ident.as_str()) {
        Some(keyword) => keyword.to_string(),
        None => IDENT.to_string(),
    }
}

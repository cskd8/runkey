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

pub const KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
};

pub fn lookup_ident(ident: String) -> String {
    match KEYWORDS.get(&ident.as_str()) {
        Some(keyword) => keyword.to_string(),
        None => IDENT.to_string(),
    }
}

use crate::token::*;

#[derive(Debug, Clone)]
pub struct Lexer {
	pub input: String,
	pub position: usize,
	pub read_position: usize,
	pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

	fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = '\0';
		} else {
			self.ch = self.input.chars().nth(self.read_position).unwrap();
		}
		self.position = self.read_position;
		self.read_position += 1;
	}

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

	pub fn new_token (&self, token_type: String, ch: char) -> token::Token {
		token::Token {
			r#type: token_type,
			literal: ch.to_string(),
		}
	}

	pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
		let tok: token::Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token::Token {
                        r#type: token::EQ.to_string(),
                        literal,
                    }
                } else {
                    self.new_token(token::ASSIGN.to_string(), self.ch)
                }
            },
            ';' => self.new_token(token::SEMICOLON.to_string(), self.ch),
            '(' => self.new_token(token::LPAREN.to_string(), self.ch),
            ')' => self.new_token(token::RPAREN.to_string(), self.ch),
            ',' => self.new_token(token::COMMA.to_string(), self.ch),
            '+' => self.new_token(token::PLUS.to_string(), self.ch),
            '-' => self.new_token(token::MINUS.to_string(), self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token::Token {
                        r#type: token::NOT_EQ.to_string(),
                        literal,
                    }
                } else {
                    self.new_token(token::BANG.to_string(), self.ch)
                }
            }
            '*' => self.new_token(token::ASTERISK.to_string(), self.ch),
            '/' => self.new_token(token::SLASH.to_string(), self.ch),
            '<' => self.new_token(token::LT.to_string(), self.ch),
            '>' => self.new_token(token::GT.to_string(), self.ch),
            '{' => self.new_token(token::LBRACE.to_string(), self.ch),
            '}' => self.new_token(token::RBRACE.to_string(), self.ch),
            '\0' => token::Token{
                r#type: token::EOF.to_string(),
                literal: "".to_string(),
            },
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier(); 
                    return token::Token {
                        literal: literal.clone(),
                        r#type: token::lookup_ident(literal),
                    }
                } else if self.is_digit(self.ch) {
                    let literal = self.read_number();
                    return token::Token {
                        r#type: token::INT.to_string(),
                        literal: literal.clone(),
                    }
                } else {
                    self.new_token(token::ILLEGAL.to_string(), self.ch)
                }
            }
        };

		self.read_char();
		tok
	}

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(&self, ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }
}

#[cfg(test)]
mod tests {
    use crate::token::*;
    use crate::lexer::*;
	#[test]
	fn next_token() {
		let input = String::from("let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
");
		let tests = vec![
            token::Token {
                r#type: token::LET.to_string(),
                literal: "let".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "five".to_string(),
            },
            token::Token {
                r#type: token::ASSIGN.to_string(),
                literal: "=".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "5".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::LET.to_string(),
                literal: "let".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "ten".to_string(),
            },
            token::Token {
                r#type: token::ASSIGN.to_string(),
                literal: "=".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::LET.to_string(),
                literal: "let".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "add".to_string(),
            },
            token::Token {
                r#type: token::ASSIGN.to_string(),
                literal: "=".to_string(),
            },
            token::Token {
                r#type: token::FUNCTION.to_string(),
                literal: "fn".to_string(),
            },
            token::Token {
                r#type: token::LPAREN.to_string(),
                literal: "(".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "x".to_string(),
            },
            token::Token {
                r#type: token::COMMA.to_string(),
                literal: ",".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "y".to_string(),
            },
            token::Token {
                r#type: token::RPAREN.to_string(),
                literal: ")".to_string(),
            },
            token::Token {
                r#type: token::LBRACE.to_string(),
                literal: "{".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "x".to_string(),
            },
            token::Token {
                r#type: token::PLUS.to_string(),
                literal: "+".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "y".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::RBRACE.to_string(),
                literal: "}".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::LET.to_string(),
                literal: "let".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "result".to_string(),
            },
            token::Token {
                r#type: token::ASSIGN.to_string(),
                literal: "=".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "add".to_string(),
            },
            token::Token {
                r#type: token::LPAREN.to_string(),
                literal: "(".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "five".to_string(),
            },
            token::Token {
                r#type: token::COMMA.to_string(),
                literal: ",".to_string(),
            },
            token::Token {
                r#type: token::IDENT.to_string(),
                literal: "ten".to_string(),
            },
            token::Token {
                r#type: token::RPAREN.to_string(),
                literal: ")".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::BANG.to_string(),
                literal: "!".to_string(),
            },
            token::Token {
                r#type: token::MINUS.to_string(),
                literal: "-".to_string(),
            },
            token::Token {
                r#type: token::SLASH.to_string(),
                literal: "/".to_string(),
            },
            token::Token {
                r#type: token::ASTERISK.to_string(),
                literal: "*".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "5".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "5".to_string(),
            },
            token::Token {
                r#type: token::LT.to_string(),
                literal: "<".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            },
            token::Token {
                r#type: token::GT.to_string(),
                literal: ">".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "5".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::IF.to_string(),
                literal: "if".to_string(),
            },
            token::Token {
                r#type: token::LPAREN.to_string(),
                literal: "(".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "5".to_string(),
            },
            token::Token {
                r#type: token::LT.to_string(),
                literal: "<".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            },
            token::Token {
                r#type: token::RPAREN.to_string(),
                literal: ")".to_string(),
            },
            token::Token {
                r#type: token::LBRACE.to_string(),
                literal: "{".to_string(),
            },
            token::Token {
                r#type: token::RETURN.to_string(),
                literal: "return".to_string(),
            },
            token::Token {
                r#type: token::TRUE.to_string(),
                literal: "true".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::RBRACE.to_string(),
                literal: "}".to_string(),
            },
            token::Token {
                r#type: token::ELSE.to_string(),
                literal: "else".to_string(),
            },
            token::Token {
                r#type: token::LBRACE.to_string(),
                literal: "{".to_string(),
            },
            token::Token {
                r#type: token::RETURN.to_string(),
                literal: "return".to_string(),
            },
            token::Token {
                r#type: token::FALSE.to_string(),
                literal: "false".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::RBRACE.to_string(),
                literal: "}".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            },
            token::Token {
                r#type: token::EQ.to_string(),
                literal: "==".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "10".to_string(),
            }, 
            token::Token {
                r#type: token::NOT_EQ.to_string(),
                literal: "!=".to_string(),
            },
            token::Token {
                r#type: token::INT.to_string(),
                literal: "9".to_string(),
            },
            token::Token {
                r#type: token::SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            token::Token {
                r#type: token::EOF.to_string(),
                literal: "".to_string(),
            },
        ];
		let mut l = lexer::Lexer::new(input);
		for tt in tests {
			let tok = l.next_token();
			assert_eq!(tok.r#type, tt.r#type);
			assert_eq!(tok.literal, tt.literal);
		}
	}
}

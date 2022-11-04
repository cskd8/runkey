use crate::token::*;

#[derive(Debug, Clone)]
struct Lexer {
	pub input: String,
	pub position: usize,
	pub read_position: usize,
	pub ch: char,
}

fn new(input: String) -> Lexer {
	let mut l = Lexer {
		input,
		position: 0,
		read_position: 0,
		ch: '\0',
	};
	l.read_char();
	l
}

impl Lexer {
	fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = '\0';
		} else {
			self.ch = self.input.chars().nth(self.read_position).unwrap();
		}
		self.position = self.read_position;
		self.read_position += 1;
	}

	pub fn new_token (&self, token_type: String, ch: char) -> token::Token {
		token::Token {
			r#type: token_type,
			literal: ch.to_string(),
		}
	}

	fn next_token(&mut self) -> token::Token {
		let tok: token::Token = match self.ch {
            '=' => self.new_token(token::ASSIGN.to_string(), self.ch),
            ';' => self.new_token(token::SEMICOLON.to_string(), self.ch),
            '(' => self.new_token(token::LPAREN.to_string(), self.ch),
            ')' => self.new_token(token::RPAREN.to_string(), self.ch),
            ',' => self.new_token(token::COMMA.to_string(), self.ch),
            '+' => self.new_token(token::PLUS.to_string(), self.ch),
            '{' => self.new_token(token::LBRACE.to_string(), self.ch),
            '}' => self.new_token(token::RBRACE.to_string(), self.ch),
            '\0' => token::Token{
                r#type: token::EOF.to_string(),
                literal: "".to_string(),
            },
            _ => token::Token{
                r#type: token::EOF.to_string(),
                literal: "".to_string(),
            },
        };

		self.read_char();
		tok
	}
}

#[cfg(test)]
mod tests {
    use crate::token::*;
    use crate::lexer::*;
	#[test]
	fn next_token() {
		let input = String::from("=+(){},;");
		let tests = vec![
			{ token::Token { r#type: token::ASSIGN.to_string(), literal: String::from("=") } },
			{ token::Token { r#type: token::PLUS.to_string(), literal: String::from("+") } },
			{ token::Token { r#type: token::LPAREN.to_string(), literal: String::from("(") } },
			{ token::Token { r#type: token::RPAREN.to_string(), literal: String::from(")") } },
			{ token::Token { r#type: token::LBRACE.to_string(), literal: String::from("{") } },
			{ token::Token { r#type: token::RBRACE.to_string(), literal: String::from("}") } },
			{ token::Token { r#type: token::COMMA.to_string(), literal: String::from(",") } },
			{ token::Token { r#type: token::SEMICOLON.to_string(), literal: String::from(";") } },
			{ token::Token { r#type: token::EOF.to_string(), literal: String::from("") } },
		];
		let mut l = lexer::new(input);
		for tt in tests {
			let tok = l.next_token();
			assert_eq!(tok.r#type, tt.r#type);
			assert_eq!(tok.literal, tt.literal);
		}
	}
}

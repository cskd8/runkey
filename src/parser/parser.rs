use crate::token::*;
use crate::lexer::*;
use crate::ast::*;

pub struct Parser {
    pub l: lexer::Lexer,
    pub cur_token: token::Token,
    pub peek_token: token::Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let mut p = Parser {
            l: l,
            cur_token: token::Token {
                literal: "".to_string(),
                r#type: token::ILLEGAL.to_string(),
            },
            peek_token: token::Token {
                literal: "".to_string(),
                r#type: token::ILLEGAL.to_string(),
            },
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        while self.cur_token.r#type != token::EOF.to_string() {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.push(s);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.r#type.as_str() {
            token::LET => Some(self.parse_let_statement()),
            token::RETURN => Some(self.parse_return_statement()),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> ast::Statement {
        if !self.expect_peek(token::IDENT) {
            return ast::Statement::EmptyStatement;
        }

        let ident = ast::Identifier {
            value: self.cur_token.literal.clone()
        };

        if !self.expect_peek(token::ASSIGN) {
            return ast::Statement::EmptyStatement;
        }

        while self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        ast::Statement::LetStatement (ident, ast::Expression::EmptyExpression)
    }

    pub fn parse_return_statement(&mut self) -> ast::Statement {
        self.next_token();

        while self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        ast::Statement::ReturnStatement (ast::Expression::EmptyExpression)
    }

    pub fn cur_token_is(&self, t: &str) -> bool {
        self.cur_token.r#type == t.to_string()
    }

    pub fn peek_token_is(&self, t: &str) -> bool {
        self.peek_token.r#type == t.to_string()
    }

    pub fn expect_peek(&mut self, t: &str) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_error(&mut self, t: &str) {
        let msg = format!("expected next token to be {}, got {} instead", t, self.peek_token.r#type);
        self.errors.push(msg);
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::parser::*;
    use crate::ast::*;

    #[test]
    fn let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let l = lexer::Lexer::new(input.to_string());
        let mut p = parser::Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
        }
        let tests = vec![
            ("x", 5),
            ("y", 10),
            ("foobar", 838383),
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            if !test_let_statement(stmt, tt.0) {
                panic!("test_let_statement failed");
            }
        }
    }

    #[test]
    fn return_statements() {
        let input = "
return 5;
return 10;
return 993322;
";
        let l = lexer::Lexer::new(input.to_string());
        let mut p = parser::Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
        }

        for stmt in program.statements {
            if let ast::Statement::ReturnStatement(_expr) = stmt {
                continue;
            }
            panic!("stmt not ast::Statement::ReturnStatement. got={:?}", stmt);
        }
    }

    fn test_let_statement(s: &ast::Statement, name: &str) -> bool {
        if let ast::Statement::LetStatement(ident, _expr) = s {
            if ident.value != name {
                panic!("s.Name not {}. got={}", name, ident.value);
            }
        } else {
            panic!("s not a LetStatement. got={:?}", s);
        }
        true
    }

    fn check_parser_errors(p: &parser::Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for msg in errors {
            eprintln!("parser error: {}", msg);
        }
        panic!();
    }
}

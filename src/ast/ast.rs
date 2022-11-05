#[derive(Debug, Clone)]
pub enum Statement {
    EmptyStatement,
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    EmptyExpression,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String
}

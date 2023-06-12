use crate::lexer::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn name(&self) -> &Identifier;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program  {
    pub statements: Vec<Box<dyn Statement>>
}

impl Program {
    pub fn new() -> Self {
        let v = Vec::new();
        Program{
            statements: v 
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str{
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        ""
    }
}

pub struct Identifier {
    token: Token,
    value: String 
}

impl Identifier {
    pub fn new(token: Token, value: String) ->Self{
        Self{
            token,
            value
        }
    }

    
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str{
        self.token.literal()
    }
}


pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    // value: dyn Expression
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier) ->Self {
        LetStatement{
            token,
            name
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }

    fn name(&self) -> &Identifier {
        &self.name
    }
}


impl Node for LetStatement {
    fn token_literal(&self) -> &str{
        self.token.literal()
    }
}

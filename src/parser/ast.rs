#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        initial: Expression,
    }
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    Nil,
}

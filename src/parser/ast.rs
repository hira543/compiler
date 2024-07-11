#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    FunctionDef {
        name: String,
        params: Vec<(String, String)>,
        body: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    IfExpr {
        condition: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Option<Box<Expr>>,
    },
    WhileLoop {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    Assignment {
        name: String,
        type_decl: Option<String>,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Literal(Literal),
    Variable(String),
    Block(Vec<Expr>),
    Return(Box<Expr>),
    Print(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    I32(i32),
    I64(i64),
    String(String),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
}

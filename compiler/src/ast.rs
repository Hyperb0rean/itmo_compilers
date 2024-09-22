#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Var(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Let {
        name: String,
        var_type: VarType,
        expr: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Option<Vec<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    ExprStmt(Box<Expr>), 
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
    I64,
    Bool,
}

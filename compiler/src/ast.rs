// src/ast.rs

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
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
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        var_type: VarType,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    ExprStmt(Expr),
}

#[derive(Debug, PartialEq)]
pub enum VarType {
    Int,
    Bool,
}

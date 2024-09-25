/*
<program> ::= <statement_list>

<statement_list> ::= <statement>*

<statement> ::= <let_statement>
              | <assign_statement>
              | <if_expression>
              | <while_expression>

<let_statement> ::= <let_expression> ";"

<assign_statement> ::=  <assign_expression> ";"

<block> ::= "{" <statement_list> "}"

<expression> ::= <literal>
               | <identifier>
               | <binary_expression>
               | <unary_expression>
               | <let_expression>
               | <if_expression>
               | <while_expression>
               | <assign_expression>

<binary_expression> ::= <expression> <binary_operator> <expression>
<binary_operator> ::= "+" | "-" | "*" | "/" | "%" 
                   | "&&" | "||" 
                   | "==" | "!=" | "<" | ">" | "<=" | ">="

<unary_expression> ::= <unary_operator> <expression>
<unary_operator> ::= "!" | "-"

<let_expression> ::= "let" <identifier> ":" <type> "=" <expression>

<if_expression> ::= "if" <expression> <block> ("else" <block>)?

<while_expression> ::= "while" <expression> <block>

<assign_expression> ::= <identifier> "=" <expression>

<literal> ::= <number> | <boolean>
<number> ::= <digit>+
<boolean> ::= "true" | "false"

<identifier> ::= <letter> (<letter> | <digit>)*

<type> ::= "num" | "bool"

<digit> ::= [0-9]
<letter> ::= [a-zA-Z_]

 */



#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i32),
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
    Number,
    Bool,
}

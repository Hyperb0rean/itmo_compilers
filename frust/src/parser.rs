use crate::ast::*;
use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

type TokenIter = Peekable<IntoIter<Token>>;

pub struct Parser<'a> {
    tokens: TokenIter,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();
        while self.tokens.peek().is_some() {
            let expr = self.parse_expression()?;
            expressions.push(expr);
        }
        Ok(expressions)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logical_and()?;

        while let Some(Token::OpOr) = self.tokens.peek() {
            self.tokens.next(); // Consume ||
            let right = self.parse_logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_equality()?;

        while let Some(Token::OpAnd) = self.tokens.peek() {
            self.tokens.next(); // Consume &&
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::OpEq) => BinaryOp::Eq,
                Some(Token::OpNeq) => BinaryOp::Neq,
                _ => break,
            };
            self.tokens.next(); // Consume operator == / !=
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_addition()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::OpLt) => BinaryOp::Lt,
                Some(Token::OpGt) => BinaryOp::Gt,
                Some(Token::OpLe) => BinaryOp::Le,
                Some(Token::OpGe) => BinaryOp::Ge,
                _ => break,
            };
            self.tokens.next(); // Consume operator of comparison
            let right = self.parse_addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplication()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::OpAdd) => BinaryOp::Add,
                Some(Token::OpSub) => BinaryOp::Sub,
                _ => break,
            };
            self.tokens.next(); // Consume operator + / -
            let right = self.parse_multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::OpMul) => BinaryOp::Mul,
                Some(Token::OpDiv) => BinaryOp::Div,
                _ => break,
            };
            self.tokens.next(); // Consume operator * / /
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(Token::OpNot) = self.tokens.peek() {
            self.tokens.next(); // Consume !
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }

        if let Some(Token::OpSub) = self.tokens.peek() {
            self.tokens.next(); // Consume -
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }

        self.parse_kw_or_expr()
    }

    fn parse_kw_or_expr(&mut self) -> Result<Expr, String> {
        match self.tokens.peek() {
            Some(Token::KwLet) => self.parse_let(),
            Some(Token::KwIf) => self.parse_if(),
            Some(Token::KwWhile) => self.parse_while(),
            _ => self.parse_simple_expr_or_literal(),
        }
    }
    fn parse_simple_expr_or_literal(&mut self) -> Result<Expr, String> {
        match self.tokens.next() {
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("Expected ')'".to_string()),
                }
            }
            Some(Token::LitNumber(n)) => Ok(Expr::Number(n)),
            Some(Token::LitBool(v)) => Ok(Expr::Bool(v)),
            Some(Token::Identifier(name)) => {
                if let Some(Token::Assign) = self.tokens.peek() {
                    self.tokens.next(); // Consume '='
                    let expr = self.parse_expr()?;

                    match self.tokens.next() {
                        Some(Token::Semicolon) => (),
                        _ => return Err("Expected ';' at the end of statement".to_string()),
                    }

                    Ok(Expr::Assign {
                        name,
                        expr: Box::new(expr),
                    })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Some(tok) => Err(format!("Unexpected token {:?}", tok)),
            None => Err("Unexpected EOF".to_string()),
        }
    }

    fn parse_let(&mut self) -> Result<Expr, String> {
        self.tokens.next(); // Consume let

        let name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err("Expected identifier after 'let'".to_string()),
        };

        match self.tokens.next() {
            Some(Token::Colon) => (),
            _ => return Err("Expected ':' after identifier".to_string()),
        }
        let var_type = match self.tokens.next() {
            Some(Token::TypeNumber) => VarType::Number,
            Some(Token::TypeBool) => VarType::Bool,
            _ => return Err("Expected type ':'".to_string()),
        };

        match self.tokens.next() {
            Some(Token::Assign) => (),
            _ => return Err("Expected operator '=' after type".to_string()),
        }

        let expr = self.parse_expr()?;

        match self.tokens.next() {
            Some(Token::Semicolon) => (),
            _ => return Err("Expected ';' at the end of statement".to_string()),
        }

        Ok(Expr::Let {
            name,
            var_type,
            expr: Box::new(expr),
        })
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.tokens.next(); // Consume if

        let condition = self.parse_expr()?;

        match self.tokens.next() {
            Some(Token::LBrace) => (),
            _ => return Err("Expected '{' after 'if'".to_string()),
        }

        let then_branch = self.parse_block()?;

        let else_branch = if let Some(Token::KwElse) = self.tokens.peek() {
            self.tokens.next(); // Consume else

            match self.tokens.next() {
                Some(Token::LBrace) => (),
                _ => return Err("Expected '{' after 'else'".to_string()),
            }

            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Expr, String> {
        self.tokens.next(); // Consume while

        let condition = self.parse_expr()?;

        match self.tokens.next() {
            Some(Token::LBrace) => (),
            _ => return Err("Expected '{' after 'while'".to_string()),
        }

        let body = self.parse_block()?;

        Ok(Expr::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if *token == Token::RBrace {
                self.tokens.next(); // Consume }
                break;
            }
            let expr = self.parse_expression()?;
            expressions.push(expr);
        }
        Ok(expressions)
    }
}

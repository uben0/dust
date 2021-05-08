#[derive(Debug)]
pub enum Expr {
    Value(i32),
    Ident(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    Seq(Vec<Stmt>, Option<Box<Self>>),
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    Mut(String, Expr),
    Eval(Expr),
    While(Expr, Expr),
    If(Expr, Expr),
    Print(Expr),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Le,
    LeEq,
    Gr,
    GrEq,
}
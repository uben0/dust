pub mod env;
use env::*;

#[derive(Debug)]
pub enum Expr {
    Value(i32),
    Ident(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    Seq(Vec<Stmt>, Option<Box<Self>>),
}
impl Expr {
    pub fn eval(&self, stack: &mut EnvFrame<i32>) -> i32 {
        match self {
            Self::Value(v) => *v,
            Self::Ident(i) => stack.get(i).unwrap_or_else(
                || panic!("value {:?} not found in stack", i)
            ).clone(),
            Self::BinOp(o, l, r) => o.eval(l.eval(stack), r.eval(stack)),
            Self::Seq(s, v) => {
                let mut stack = EnvFrame::new(Some(stack));
                s.iter().for_each(|s| s.eval(&mut stack));
                if let Some(v) = v {
                    v.eval(&mut stack)
                } else {
                    0
                }
            }
        }
    }
    // pub fn reduce(self, env: &dyn Env<Option<i32>>) -> Self {
    //     match self {
    //         Self::Value(v) => Self::Value(v),
    //         Self::Ident(i) => match env.get(&i).unwrap_or_else(
    //             || panic!("value {:?} not found in stack", i)
    //         ) {
    //             Some(v) => Self::Value(v),
    //             None    => Self::Ident(i),
    //         }
    //         Self::BinOp(o, l, r) => {
    //             match (l.reduce(env), r.reduce(env)) {
    //                 (Self::Value(l), Self::Value(r)) => Self::Value(o.eval(l, r)),
    //                 (l, r) => Self::BinOp(o, l, r),
    //             }
    //         }
    //         Self::Seq(s, v) => {
    //             let mut env = EnvFrame::new(Some(env));
    //             s.iter().for_each(|s| s.eval(&mut stack));
    //             if let Some(v) = v {
    //                 v.eval(&mut stack)
    //             } else {
    //                 0
    //             }
    //         }
    //     }
    // }
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
impl Stmt {
    fn eval(&self, stack: &mut EnvFrame<i32>) {
        match self {
            Self::Let(i, e) => {
                let val = e.eval(stack);
                stack.set(i.to_owned(), val);
            }
            Self::Mut(i, e) => {
                *stack.get_mut(i).unwrap_or_else(
                    || panic!("value {:?} not found in scope", i)
                ) = e.eval(stack);
            }
            Self::Eval(e) => {
                e.eval(stack);
            }
            Self::Print(e) => {
                println!("{:?}", e.eval(stack));
            }
            Self::While(c, b) => {
                while c.eval(stack) != 0 {
                    b.eval(stack);
                }
            }
            Self::If(c, b) => {
                if c.eval(stack) != 0 {
                    b.eval(stack);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
impl BinOp {
    fn eval(self, l: i32, r: i32) -> i32 {
        match self {
            Self::Add   => l + r,
            Self::Sub   => l - r,
            Self::Mul   => l * r,
            Self::Div   => l / r,
            Self::Eq    => if l == r {1} else {0},
            Self::NotEq => if l != r {1} else {0},
            Self::Le    => if l <  r {1} else {0},
            Self::LeEq  => if l <= r {1} else {0},
            Self::Gr    => if l >  r {1} else {0},
            Self::GrEq  => if l >= r {1} else {0},
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum Value {
//     Int(i32),
//     Bool(bool),
// }
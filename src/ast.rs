pub mod env;
use env::*;

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Ident(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    Seq(Vec<Stmt>, Option<Box<Self>>),
}
impl Expr {
    pub fn eval(&self, stack: &mut EnvFrame<Value>) -> Value {
        match self {
            Self::Value(v) => v.clone(),
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
                    Value::Int(0)
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
    fn eval(&self, stack: &mut EnvFrame<Value>) {
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
                while c.eval(stack).downcast_bool() {
                    b.eval(stack);
                }
            }
            Self::If(c, b) => {
                if c.eval(stack).downcast_bool() {
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
    fn symbol(self) -> &'static str {
        match self {
            Self::Add   => "+",
            Self::Sub   => "-",
            Self::Mul   => "*",
            Self::Div   => "/",
            Self::Eq    => "==",
            Self::NotEq => "!=",
            Self::Le    => "<",
            Self::LeEq  => "<=",
            Self::Gr    => ">",
            Self::GrEq  => ">=",
        }
    }
    fn eval(self, l: Value, r: Value) -> Value {
        use Value::*;
        match (self, l, r) {
            (Self::Add  , Int( l), Int( r)) => Int( l +  r),
            (Self::Sub  , Int( l), Int( r)) => Int( l -  r),
            (Self::Mul  , Int( l), Int( r)) => Int( l *  r),
            (Self::Div  , Int( l), Int( r)) => Int( l /  r),
            (Self::Eq   , Int( l), Int( r)) => Bool(l == r),
            (Self::NotEq, Int( l), Int( r)) => Bool(l != r),
            (Self::Le   , Int( l), Int( r)) => Bool(l <  r),
            (Self::LeEq , Int( l), Int( r)) => Bool(l <= r),
            (Self::Gr   , Int( l), Int( r)) => Bool(l >  r),
            (Self::GrEq , Int( l), Int( r)) => Bool(l >= r),
            (Self::Eq   , Bool(l), Bool(r)) => Bool(l == r),
            (Self::NotEq, Bool(l), Bool(r)) => Bool(l != r),
            (o, l, r) => panic!("undefined operation: {} {} {}", l.type_name(), o.symbol(), r.type_name()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}
impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Self::Int( ..) => "int",
            Self::Bool(..) => "bool",
        }
    }
    fn downcast_bool(self) -> bool {
        match self {
            Self::Bool(v) => v,
            v => panic!("failed to dowcast value {:?} to bool", v),
        }
    }
}
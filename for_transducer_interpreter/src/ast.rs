pub enum Stmt {
    Print(Expr),
    For(String, bool, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Vec<Stmt>), // If statement with condition, then branch, else branch
}

pub enum Expr {
    Number(i32),
    Var(String),
    Str(String),
    LessEqual(Box<Expr>, Box<Expr>), // Less than or equal to comparison
    Less(Box<Expr>, Box<Expr>),      // Less than comparison
    Equal(Box<Expr>, Box<Expr>),     // Equal to comparison
    NotEqual(Box<Expr>, Box<Expr>),  // Not equal to comparison
    Label(String),
}

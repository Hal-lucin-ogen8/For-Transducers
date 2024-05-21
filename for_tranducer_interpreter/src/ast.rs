#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Var(String),
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    For(String, i32, i32, Vec<Stmt>),
}

pub enum Stmt {
    Print(Pexpr), // 
    For0(String, Vec<Stmt>), //first to last
    For1(String, Vec<Stmt>), //last to first
    If(Bexpr, Vec<Stmt>), // If statement with condition, then branch
}


pub enum Pexpr {
    Label(String),
    Str(String),
}

#[derive(Clone)]
pub enum Bexpr {
    Var(String),
    Str(String),
    LessEqual(Box<Bexpr>, Box<Bexpr>), // Less than or equal to comparison
    Less(Box<Bexpr>, Box<Bexpr>),      // Less than comparison
    Equal(Box<Bexpr>, Box<Bexpr>),     // Equal to comparison
    NotEqual(Box<Bexpr>, Box<Bexpr>),  // Not equal to comparison
    GreaterEqual(Box<Bexpr>, Box<Bexpr>), // Greater than or equal to comparison
    Greater(Box<Bexpr>, Box<Bexpr>),      // Greater than comparison
    Not(Box<Bexpr>),
    Label(String),
}


pub enum Fexpr {
    Number(i32),
    Str(String),
    Var(String),
    Label(String),
}



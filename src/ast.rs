// hcc ast grammars

// Decl is any static declaration (functions, classes, etc...)
#[derive(PartialEq, Clone, Debug)]
pub enum Decl {
    Class(ClassDecl),
    Def(DefDecl),
    Func(FuncDecl),
    Import(ImportDecl),
    Null(NullDecl)
}

// Stmt is anything that *does* something
#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Asm(AsmStmt),
    Break(BreakStmt),
    Compound(CompoundStmt),
    Expr(Expr),
    For(ForStmt),
    FuncCall(FuncCall),
    If(IfStmt),
    Ret(RetStmt),
    SwitchCase(SwitchCase),
    Switch(SwitchStmt),
    While(WhileStmt),
    Value(ValueStmt)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    BinaryOperator(BinaryOperator)
}

#[derive(PartialEq, Clone, Debug)]
pub struct ClassDecl {
    pub name: String,
    pub access: Access,
    pub vars: Vec(ValueStmt),
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValueStmt {
    UnInit
    Init
    Union(Union
}

#[derive(PartialEq, Clone, Debug)]
pub struct FuncDecl {
    pub name: String,
    pub args: Vec<String>
}

#[derive(PartialEq, Clone, Debug)]
pub enum Access {
    Public,
    Protected,
    Private
}

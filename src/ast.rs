use std::string::ToString;
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug, Copy, Clone)]
pub enum FuncType {
    Int,
}

impl ToString for FuncType {
    fn to_string(&self) -> String {
        match self {
            &FuncType::Int => "i32".to_string(),
            _ => "not defined".to_string(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub stmt: Stmt,
}
#[derive(Debug, Copy, Clone)]
pub struct Stmt {
    pub num: i32,
}

use crate::ast::*;
use std::fmt;
pub struct IR {
    pub content: String,
}

impl IR {
    pub fn with_ast(compunit: &CompUnit) -> Self {
        let funcdef = compunit.func_def.clone();
        let tab = "  ";
        let lb = "{";
        let rb = "}";
        IR {
            content: format!(
                "fun @{}(): {} {lb}\n%entry:\n{tab}ret {}\n{rb}",
                funcdef.ident,
                funcdef.func_type.to_string(),
                funcdef.block.stmt.num,
            ),
        }
    }
}

impl fmt::Display for IR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

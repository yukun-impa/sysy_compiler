mod ast;
mod ir;
use ir::*;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();
    let input = read_to_string(input)?;

    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
    let ir = IR::with_ast(&ast);

    let driver = koopa::front::Driver::from(ir.content);
    let program = driver.generate_program().unwrap();
    for &func in program.func_layout() {
        let func_data = program.func(func);
    }
    Ok(())
}

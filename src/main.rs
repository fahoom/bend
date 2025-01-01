#[derive(Debug)]
pub struct Function {
    blocks: Vec<BasicBlock>,
}

#[derive(Debug)]
pub struct BasicBlock {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Return { val: u64 },
}

fn main() {
    let func = Function {
        blocks: vec![BasicBlock {
            instructions: vec![Instruction::Return { val: 0 }],
        }],
    };

    println!("{:#?}", func);
}

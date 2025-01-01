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
    Constant { val: u64 },
    Binary { op: BinOp, lhs: usize, rhs: usize },
    Return { val: usize },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
}

fn main() {
    // In code, this would be ```fn calculate() { return 9 + 10; }````
    let func = Function {
        blocks: vec![BasicBlock {
            instructions: vec![
                Instruction::Constant { val: 9 },
                Instruction::Constant { val: 10 },
                Instruction::Binary {
                    op: BinOp::Add,
                    lhs: 0,
                    rhs: 1,
                },
                Instruction::Return { val: 2 },
            ],
        }],
    };

    println!("{:#?}", func);
}

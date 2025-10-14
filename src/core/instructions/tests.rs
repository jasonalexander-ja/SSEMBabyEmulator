use super::*;


fn get_operation_desc() -> Vec<(BabyInstruction, String)> {
    vec![
        (BabyInstruction::Jump(0), "jump instruction".to_owned()),
        (BabyInstruction::RelativeJump(0), "relative jump instruction".to_owned()),
        (BabyInstruction::Negate(0), "negate instruction".to_owned()),
        (BabyInstruction::Store(0), "store instruction".to_owned()),
        (BabyInstruction::Subtract(0), "subtract instruction".to_owned()),
        (BabyInstruction::SkipNextIfNegative, "skip next if negative instruction".to_owned()),
        (BabyInstruction::Stop, "stop instruction".to_owned()),
        (BabyInstruction::AbsoluteValue(5), format!("absolute value 5"))
    ]
}

fn get_number_to_instruction() -> Vec<(u16, BabyInstruction)> {
    vec![
        ((0b000 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Jump(5)),
        ((0b100 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::RelativeJump(5)),
        ((0b010 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Negate(5)),
        ((0b110 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Store(5)),
        ((0b001 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Subtract(5)),
        ((0b101 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Subtract(5)),
        ((0b011 << (INSTR_LEN - 3)) + 0b000, BabyInstruction::SkipNextIfNegative),
        ((0b111 << (INSTR_LEN - 3)) + 0b000, BabyInstruction::Stop),
    ]
}

fn get_instruction_to_no() -> Vec<(WORD, BabyInstruction)> {
    vec![
        ((0b000 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Jump(5)),
        ((0b100 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::RelativeJump(5)),
        ((0b010 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Negate(5)),
        ((0b110 << (INSTR_LEN - 3)) + 0b101, BabyInstruction::Store(5)),
        ((0b011 << (INSTR_LEN - 3)) + 0b000, BabyInstruction::SkipNextIfNegative),
        ((0b111 << (INSTR_LEN - 3)) + 0b000, BabyInstruction::Stop),
    ]
}

fn get_instructions_with_operands(op: u16, res: usize) -> Vec<(BabyInstruction, usize)> {
    vec![
        (BabyInstruction::Jump(op), res),
        (BabyInstruction::RelativeJump(op), res),
        (BabyInstruction::Negate(op), res),
        (BabyInstruction::Store(op), res),
        (BabyInstruction::Subtract(op), res),
        (BabyInstruction::SkipNextIfNegative, 0),
        (BabyInstruction::Stop, 0),
        (BabyInstruction::AbsoluteValue(0), 0),
    ]
}

#[test]
fn tests_get_instr_description() {
    get_operation_desc().iter().for_each(|(i, r)| {
        assert_eq!(i.get_instr_description(), *r);
    });
}

#[test]
fn test_from_number() {
    get_number_to_instruction().iter().for_each(|(n, i)| {
        assert_eq!(BabyInstruction::from_number(*n), *i);
    });
}

#[test]
fn test_to_number() {
    get_instruction_to_no().iter().for_each(|(n, i)| {
        assert_eq!(i.to_number(), *n);
    });
    assert!(
        BabyInstruction::Subtract(5).to_number() == (0b001 << (INSTR_LEN - 3)) + 0b0101 || 
        BabyInstruction::Subtract(5).to_number() == (0b101 << (INSTR_LEN - 3)) + 0b0101
    );
}

#[test]
fn test_to_numbers() {
    let vec = vec![BabyInstruction::Jump(5); 5];
    let numbers = BabyInstruction::to_numbers(vec);
    assert_eq!(numbers[0], 5);
    assert_eq!(numbers[4], 5);
    assert_eq!(numbers[31], 0);
}

#[test]
fn test_to_numbers_longer() {
    let vec = vec![BabyInstruction::Jump(5); 50];
    let numbers = BabyInstruction::to_numbers(vec);
    assert_eq!(numbers[0], 5);
    assert_eq!(numbers[4], 5);
    assert_eq!(numbers[31], 5);
}

#[test]
fn test_get_operand_in_range() {
    get_instructions_with_operands(12, 12).iter().for_each(|(o, r)| {
        assert_eq!(o.get_operand(), *r);
    });
}

#[test]
fn test_get_operand_out_range() {
    get_instructions_with_operands(33, 1).iter().for_each(|(o, r)| {
        assert_eq!(o.get_operand(), *r);
    });
}
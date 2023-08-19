use super::*;


fn get_intrsuctions<'a>() -> Vec<(&'a str, Result<Instruction, InstructionError>)> {
    let v = 10;
    vec![
        ("jmp 0xA", Ok(Instruction::Jump(Value::Value(v)))),
        ("jrp 0xA", Ok(Instruction::RelativeJump(Value::Value(v)))),
        ("ldn 0xA", Ok(Instruction::Negate(Value::Value(v)))),
        ("sto 0xA", Ok(Instruction::Store(Value::Value(v)))),
        ("sub 0xA", Ok(Instruction::Subtract(Value::Value(v)))),
        ("cmp", Ok(Instruction::Test)),
        ("stp", Ok(Instruction::Stop)),
        ("sadasdasd", Err(InstructionError::UnkownInstruction("sadasdasd".to_owned()))),
    ]
}

fn get_ogn_intrsuctions<'a>() -> Vec<(&'a str, Result<Instruction, InstructionError>)> {
    let v = 10;
    vec![
        ("0xA, Cl", Ok(Instruction::Jump(Value::Value(v)))),
        ("Add 0xA, Cl", Ok(Instruction::RelativeJump(Value::Value(v)))),
        ("-0xA, C", Ok(Instruction::Negate(Value::Value(v)))),
        ("c, 0xA", Ok(Instruction::Store(Value::Value(v)))),
        ("SUB 0xA", Ok(Instruction::Subtract(Value::Value(v)))),
        ("Test", Ok(Instruction::Test)),
        ("Stop", Ok(Instruction::Stop)),
        ("sadasdasd", Err(InstructionError::UnkownInstruction("sadasdasd".to_owned()))),
    ]
}

fn get_instructions_with_operands() -> Vec<(Instruction, Value)> {
    vec![
        (Instruction::Jump(Value::Value(10)), Value::Value(10)),
        (Instruction::RelativeJump(Value::Value(10)), Value::Value(10)),
        (Instruction::Negate(Value::Value(10)), Value::Value(10)),
        (Instruction::Store(Value::Value(10)), Value::Value(10)),
        (Instruction::Subtract(Value::Value(10)), Value::Value(10)),
        (Instruction::Test, Value::Value(0)),
        (Instruction::Stop, Value::Value(0)),
    ]
}

fn get_instructions_with_description() -> Vec<(Instruction, String)> {
    vec![
        (Instruction::Jump(Value::Value(10)), "jump".to_owned()),
        (Instruction::RelativeJump(Value::Value(10)), "relative jump".to_owned()),
        (Instruction::Negate(Value::Value(10)), "negate".to_owned()),
        (Instruction::Store(Value::Value(10)), "store".to_owned()),
        (Instruction::Subtract(Value::Value(10)), "subtract".to_owned()),
        (Instruction::Test, "test".to_owned()),
        (Instruction::Stop, "stop".to_owned()),
    ]
}

#[test]
fn test_instruction_parse() {
    get_intrsuctions().iter().for_each(|(i, r)| {
        assert_eq!(Instruction::parse(i), *r);
    });
}

#[test]
fn test_instruction_parse_ogn() {
    get_ogn_intrsuctions().iter().for_each(|(i, r)| {
        assert_eq!(Instruction::parse_ogn(i), *r);
    });
}

#[test]
fn test_get_operand() {
    get_instructions_with_operands().iter().for_each(|(i, r)| {
        assert_eq!(i.get_operand(), *r);
    });
}

#[test]
fn test_describe() {
    get_instructions_with_description().iter().for_each(|(i, r)| {
        assert_eq!(i.describe(), *r);
    });
}
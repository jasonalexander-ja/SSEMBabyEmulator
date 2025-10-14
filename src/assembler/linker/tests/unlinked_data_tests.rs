use super::*;


#[test]
fn test_get_tag() {
    let tags: HashMap<String, WORD> = HashMap::from([("foo".to_owned(), 5)]);
    match UnlinkedData::get_tag("foo", &tags) {
        Ok(v) => assert_eq!(v, 5),
        Err(_) => assert!(false)
    }
    match UnlinkedData::get_tag("bar", &tags) {
        Err(e) => assert_eq!(e, "bar".to_owned()),
        Ok(_) => assert!(false)
    }
}

#[test]
fn test_resolve_value() {
    let tags: HashMap<String, WORD> = HashMap::from([("foo".to_owned(), 5)]);
    match UnlinkedData::resolve_value(&Value::Value(5), &tags) {
        Ok(v) => assert_eq!(v, 5),
        Err(_) => assert!(false)
    }
    match UnlinkedData::resolve_value(&Value::Tag("foo".to_owned()), &tags) {
        Ok(v) => assert_eq!(v, 5),
        Err(_) => assert!(false)
    }
    match UnlinkedData::resolve_value(&Value::Tag("bar".to_owned()), &tags) {
        Err(e) => assert_eq!(e, "bar".to_owned()),
        Ok(_) => assert!(false)
    }
}

#[test]
fn test_resolve_absolute_value() {
    let tags: HashMap<String, WORD> = HashMap::from([("foo".to_owned(), 5)]);
    match UnlinkedData::resolve_absolute_value(&Value::Value(5), &tags) {
        Ok(v) => assert_eq!(v, BabyInstruction::AbsoluteValue(5)),
        Err(_) => assert!(false)
    }
    match UnlinkedData::resolve_absolute_value(&Value::Tag("foo".to_owned()), &tags) {
        Ok(v) => assert_eq!(v, BabyInstruction::AbsoluteValue(5)),
        Err(_) => assert!(false)
    }
    match UnlinkedData::resolve_absolute_value(&Value::Tag("bar".to_owned()), &tags) {
        Err(e) => assert_eq!(e, TagError::UnknownTagName("bar".to_owned())),
        Ok(_) => assert!(false)
    }
}

fn get_litteral_value_instruction(value: WORD, result: u16) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
    vec![
        (Instruction::Jump(Value::Value(value)), Ok(BabyInstruction::Jump(result))),
        (Instruction::RelativeJump(Value::Value(value)), Ok(BabyInstruction::RelativeJump(result))),
        (Instruction::Negate(Value::Value(value)), Ok(BabyInstruction::Negate(result))),
        (Instruction::Store(Value::Value(value)), Ok(BabyInstruction::Store(result))),
        (Instruction::Subtract(Value::Value(value)), Ok(BabyInstruction::Subtract(result))),
        (Instruction::Test, Ok(BabyInstruction::SkipNextIfNegative)),
        (Instruction::Stop, Ok(BabyInstruction::Stop)),
    ]
}

fn get_tag_ref_instructions(value: &str, res: u16) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
    vec![
        (Instruction::Jump(Value::Tag(String::from(value))), Ok(BabyInstruction::Jump(res))),
        (Instruction::RelativeJump(Value::Tag(String::from(value))), Ok(BabyInstruction::RelativeJump(res))),
        (Instruction::Negate(Value::Tag(String::from(value))), Ok(BabyInstruction::Negate(res))),
        (Instruction::Store(Value::Tag(String::from(value))), Ok(BabyInstruction::Store(res))),
        (Instruction::Subtract(Value::Tag(String::from(value))), Ok(BabyInstruction::Subtract(res))),
    ]
}

fn get_tag_ref_instructions_err(value: &str, err: TagError) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
    vec![
        (Instruction::Jump(Value::Tag(String::from(value))), Err(err.clone())),
        (Instruction::RelativeJump(Value::Tag(String::from(value))), Err(err.clone())),
        (Instruction::Negate(Value::Tag(String::from(value))), Err(err.clone())),
        (Instruction::Store(Value::Tag(String::from(value))), Err(err.clone())),
        (Instruction::Subtract(Value::Tag(String::from(value))), Err(err.clone())),
    ]
}

#[test]
fn test_resolve_instruction() {
    let tags: HashMap<String, WORD> = HashMap::from([("foo".to_owned(), 5)]);

    get_litteral_value_instruction(5, 5).iter().for_each(|(i, res)| {
        assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
    });

    get_tag_ref_instructions("foo", 5).iter().for_each(|(i, res)| {
        assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
    });

    get_tag_ref_instructions_err("bar", TagError::UnknownTagName("bar".to_owned())).iter().for_each(|(i, res)| {
        assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
    });
}

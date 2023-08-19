use super::*;


#[cfg(test)]
pub mod unlinked_data_tests;

#[test]
fn test_inline_tags() {
    let vec = vec![
        LineType::Tag("foo".to_owned()),
        LineType::Instruction(Instruction::Negate(Value::Tag("foo".to_owned()))),
        LineType::Instruction(Instruction::Negate(Value::Tag("foo".to_owned()))),
    ];
    let res = inline_tags(vec);
    assert_eq!(res.len(), 2);
    match &res[0] {
        (Some(tag), UnlinkedData::Instruction(Instruction::Negate(Value::Tag(tag_ref)))) => {
            assert_eq!(tag, "foo");
            assert_eq!(tag_ref, "foo")
        },
        _ => assert!(false)
    }
    match &res[1] {
        (None, UnlinkedData::Instruction(Instruction::Negate(Value::Tag(tag_ref)))) => 
            assert_eq!(tag_ref, "foo"),
        _ => assert!(false)
    }
}

#[test]
fn test_position_tags() {
    let vec: Vec<(Option<String>, UnlinkedData)> = vec![
        (Some(format!("foo1")), UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
        (None, UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
        (Some(format!("foo2")), UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
    ];

    let tags = position_tags(&vec);
    assert_eq!(tags.iter().len(), 2);
    assert_eq!(tags.get("foo1"), Some(&0));
    assert_eq!(tags.get("foo2"), Some(&2));
}

#[test]
fn test_link_tags_correct() {
    let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
    let lines: Vec<UnlinkedData> = vec![
        UnlinkedData::Instruction(Instruction::Jump(Value::Tag("foo".to_owned()))),
        UnlinkedData::Instruction(Instruction::Jump(Value::Value(5))),
    ];
    match link_tags(lines, tags) {
        Ok(res) => {
            assert_eq!(res.len(), 2);
            assert_eq!(res[0], BabyInstruction::Jump(5));
            assert_eq!(res[1], BabyInstruction::Jump(5));
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_link_tags_fail() {
    let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
    let lines: Vec<UnlinkedData> = vec![
        UnlinkedData::Instruction(Instruction::Jump(Value::Tag("bar".to_owned()))),
        UnlinkedData::Instruction(Instruction::Jump(Value::Value(5))),
    ];
    match link_tags(lines, tags) {
        Err(e) => {
            assert_eq!(e, LinkingError::TagError(TagError::UnknownTagName("bar".to_owned())))
        },
        Ok(_) => assert!(false),
    }
}

#[test]
fn test_link_parsed_lines_correct() {
    let lines: Vec<LineType> = vec![
        LineType::Tag("start".to_owned()),
        LineType::Instruction(Instruction::Negate(Value::Value(2))),
        LineType::Instruction(Instruction::Jump(Value::Tag("start".to_owned()))),
        LineType::Absolute(Value::Value(5)),
    ];
    match link_parsed_lines(lines) {
        Ok(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], BabyInstruction::Negate(2));
            assert_eq!(v[1], BabyInstruction::Jump(0));
            assert_eq!(v[2], BabyInstruction::AbsoluteValue(5));
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_link_parsed_lines_fail() {
    let lines: Vec<LineType> = vec![
        LineType::Tag("start".to_owned()),
        LineType::Instruction(Instruction::Negate(Value::Value(2))),
        LineType::Instruction(Instruction::Jump(Value::Tag("foo".to_owned()))),
        LineType::Absolute(Value::Value(5)),
    ];
    match link_parsed_lines(lines) {
        Err(e) => {
            assert_eq!(e, LinkingError::TagError(TagError::UnknownTagName("foo".to_owned())))
        },
        Ok(_) => assert!(false),
    }
}
